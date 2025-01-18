import sys
import zmq
import numpy as np
import ast
import json

from PyQt5.QtWidgets import QApplication, QVBoxLayout, QLabel, QWidget
from PyQt5.QtCore import QThread, pyqtSignal

from PyQt5.QtWidgets import (
    QApplication, QVBoxLayout, QHBoxLayout, QWidget, QPushButton, QLabel,QSpinBox
)
from PyQt5.QtCore import QThread, pyqtSignal, QTimer
from matplotlib.backends.backend_qt5agg import (
    FigureCanvasQTAgg as FigureCanvas
)
from matplotlib.figure import Figure


# First Thread to receive messages from Rust; used for slow logging
class MessageReceiver1(QThread):
    message_received = pyqtSignal(str)

    def run(self):
        # Open a new ZMQ socket to listen to Rust
        context = zmq.Context()
        socket = context.socket(zmq.PULL)
        socket.bind("tcp://*:5555")  # Bind to the same address Rust connects to

        while True:
            message = socket.recv_string()
            self.message_received.emit(message)

# Second Thread to receive messages from Rust; used for fast triggered logging
class MessageReceiver3(QThread):
    message_received = pyqtSignal(str)

    def run(self):
        # Open a new ZMQ socket to listen to Rust
        context = zmq.Context()
        socket = context.socket(zmq.PULL)
        socket.bind("tcp://*:49156")  # Bind to the same address Rust connects to

        while True:
            #Listen on socket forever, send mesages to GUI
            message = socket.recv_string()
            self.message_received.emit(message)

# Third Thread to receive FFT
class MessageReceiver2(QThread):
    message_received = pyqtSignal(str)

    def run(self):
        # Open a new ZMQ socket to listen to Rust
        context = zmq.Context()
        socket = context.socket(zmq.PULL)
        socket.bind("tcp://*:49158")  # Bind to the same address Rust connects to

        while True:
            #Listen on socket forever, send mesages to GUI
            message = socket.recv_string()
            self.message_received.emit(message)

# Main window
class DataLoggerGUI(QWidget):
    """GUI for real-time data visualization and deviation tracking."""
    def __init__(self):
        super().__init__()
        self.init_ui()

        # Data containers
        self.time_log = []
        self.time_counter = 0
        self.b_field_log = []
        self.b_field_trace = []
        self.t_trace = []
        self.fft_trace = []
        self.fft_freq_ax = [x*2000.0/512.0 for x in range(1, 257)]
        self.trigger_time = 0
        self.fresh_trace = False
        self.trace_max = 0
        self.trace_min = 0
        self.trace_mean = 0
        self.rolling_avg = []
        self.zero_reference = 0
        self.rolling_window = 1200  # Rolling window in seconds

        # Start first message receiver thread
        self.receiver1 = MessageReceiver1()
        self.receiver1.message_received.connect(self.handle_new_data)
        self.receiver1.start()

        self.receiver2 = MessageReceiver2()
        self.receiver2.message_received.connect(self.handle_fft)
        self.receiver2.start()

        '''
        # Start second message receiver thread
        self.receiver2 = MessageReceiver2()
        self.receiver2.message_received.connect(self.update_label2)
        self.receiver2.start()
        '''
        self.receiver3 = MessageReceiver3()
        self.receiver3.message_received.connect(self.handle_fast_data)
        self.receiver3.start()

        # GUI update timer
        #Throttles GUI updates to not blow up computer
        self.update_timer = QTimer()
        self.update_timer.timeout.connect(self.update_plot)
        self.update_timer.start(50)  # Update every 50ms

    def init_ui(self):
        """Initialize GUI components."""
        layout = QVBoxLayout()

        # Rolling average plot
        self.plot_canvas = FigureCanvas(Figure())
        layout.addWidget(self.plot_canvas)
        self.ax = self.plot_canvas.figure.add_subplot(311)
        self.ax.set_title("Rolling Average (1-Minute Window)")
        self.ax.set_xlabel("Time (s)")
        self.ax.set_ylabel("B-field (mG)")

        self.fast_ax = self.plot_canvas.figure.add_subplot(312)
        self.fast_ax.set_title("Triggered Scope Trace")
        self.fast_ax.set_xlabel("Time (ms)")
        self.fast_ax.set_ylabel("B-field (mG)")

        self.fft_ax = self.plot_canvas.figure.add_subplot(313)
        self.fft_ax.set_title("Triggered Scope Trace")
        self.fft_ax.set_xlabel("Freq (Hz)")
        self.fft_ax.set_ylabel("Intensity (arb.)")

        self.plot_canvas.figure.tight_layout()

        # Deviation gauge label
        self.gauge_label = QLabel("Deviation from Last Zero: 0.00 mG")
        self.gauge_label.setStyleSheet("font-size: 18px; color: blue;")
        layout.addWidget(self.gauge_label)
        

        # Rolling average gauge label
        self.mean_gauge_label = QLabel("1 Minute Rolling Avg. : 0.00 mG")
        self.mean_gauge_label.setStyleSheet("font-size: 18px; color: blue;")
        layout.addWidget(self.mean_gauge_label)


        # Buttons


        button_layout = QHBoxLayout()

        self.zero_button = QPushButton("Zero")
        self.zero_button.clicked.connect(self.set_zero_reference)
        button_layout.addWidget(self.zero_button)

        self.clear_button = QPushButton("Clear Data")
        self.clear_button.clicked.connect(self.clear_data)
        button_layout.addWidget(self.clear_button)

        spinbox_layout = QHBoxLayout()

        self.spinbox_label = QLabel("FFT Max Freq.")
        spinbox_layout.addWidget(self.spinbox_label)

        self.num_input = QSpinBox()
        self.num_input.setRange(1, 1000)  # You can set a range as needed
        self.num_input.setValue(1000)  # Default value
        spinbox_layout.addWidget(self.num_input)

        layout.addLayout(spinbox_layout)
        layout.addLayout(button_layout)
        self.setLayout(layout)
        self.setWindowTitle("North/South Magnetometer Rolling Average")

  

    def set_zero_reference(self):
        """Set the zero reference based on the current rolling average."""
        if self.rolling_avg:
            self.zero_reference = self.rolling_avg[-1]
        else:
            self.zero_reference = 0
        self.update_gauge(0)  # Reset gauge display

    def clear_data(self):
        """Clear all logged data and reset the plot."""
        self.time_log.clear()
        self.b_field_log.clear()
        self.rolling_avg.clear()
        self.ax.clear()
        self.ax.set_title("Rolling Average (1-Minute Window)")
        self.ax.set_xlabel("Time (s)")
        self.ax.set_ylabel("B-field (mG)")
        self.fast_ax.clear()
        self.fast_ax.set_title("Triggered Scope Trace")
        self.fast_ax.set_xlabel("Time (ms)")
        self.fast_ax.set_ylabel("B-field (mG)")
        self.plot_canvas.draw()

    def handle_new_data(self, batchRaw):
        """Process a batch of new data."""
        batch = ast.literal_eval(batchRaw)
        tc_old = self.time_counter
        self.time_counter = self.time_counter + len(batch)
        for k in range(tc_old, self.time_counter):
            self.time_log.append(k)
        for b_field in batch:
            if abs(b_field) < 1000:

                self.b_field_log.append(b_field)

            # Remove old data outside the rolling window
            
            while self.time_log[0] < self.time_counter - self.rolling_window:
                self.time_log.pop(0)
                self.b_field_log.pop(0)
            
            # Compute rolling average
            if self.b_field_log:
                rolling_avg_value = np.mean(self.b_field_log)
                self.rolling_avg.append(rolling_avg_value)

    def handle_fast_data(self, trace):
        """Process a batch of new data."""
        batch = json.loads(trace)
        self.b_field_trace = batch["values"]
        self.t_trace = batch["timestamps"]
        print(self.t_trace)
        self.trigger_time = batch["start_time"]
        self.trace_max = batch["max"]
        self.trace_mean = batch["mean"]
        self.trace_min = batch["min"]
        self.fresh_trace = True

    def handle_fft(self, fft):
        batch = ast.literal_eval(fft)
        self.fft_trace = batch[1:]
        
    def update_plot(self):
        """Update the plot and gauge at a throttled rate."""
        if not self.time_log:
            return
        # Update plot
        self.ax.clear()
        print(len(self.time_log))
        self.ax.plot(
            np.array([t - self.time_log[0] for t in self.time_log])*0.05,  # Relative time
            self.b_field_log,
            label="25 ms Avg",
            alpha=0.5,
        )
        self.ax.plot(
            np.array([t - self.time_log[0] for t in self.time_log])*0.05,
            self.rolling_avg[-len(self.b_field_log):],  # Rolling average
            label="Rolling Avg",
            color="red",
        )
        self.ax.legend()
        self.ax.set_title("Rolling Average (1-Minute Window)")
        self.ax.set_xlabel("Time (s)")
        self.ax.set_ylabel("B-field (mG)")

        if self.t_trace and self.fresh_trace:
            self.fresh_trace = False
            self.fast_ax.clear()
            self.fast_ax.plot(np.array(range(0,len(self.b_field_trace)))*0.5, self.b_field_trace, label="Raw B-field")
            self.fast_ax.set_title("Triggered Trace")
            self.fast_ax.set_xlabel("Time (ms)")
            self.fast_ax.set_ylabel("B-field (mG)")
            self.fast_ax.plot(np.array([0,len(self.b_field_trace)])*0.5, np.array([1,1])*self.trace_mean, label="Trace Mean: {} mG".format(round(self.trace_mean, 4)),c="g")
            #self.fast_ax.plot(np.array(range(0,len(self.b_field_trace)))*0.5, np.ones(len(self.b_field_trace))*self.trace_max, label="Trace Max: {} mG".format(round(self.trace_max, 4)), c="r")
            #self.fast_ax.plot(np.array(range(0,len(self.b_field_trace)))*0.5, np.ones(len(self.b_field_trace))*self.trace_min, label="Trace Min: {} mG".format(round(self.trace_min, 4)), c="r")
            self.fast_ax.legend()
        if self.fft_trace:
            top_in = self.num_input.value()
            top = round(top_in/1000*256) 
            self.fft_ax.clear()
            sixtyHzArr = np.array(range(60,top_in,60))
            self.fft_ax.vlines(sixtyHzArr, 0, max(self.fft_trace[0:top])+0.2, colors="r", alpha=0.2)
            self.fft_ax.plot(self.fft_freq_ax[0:top], self.fft_trace[0:top])
            self.fft_ax.set_title("FFT")
            self.fft_ax.set_xlabel("Frequency (Hz)")
            self.fft_ax.set_ylabel("Power (mG^2)")

        self.plot_canvas.draw()

        # Update deviation gauge
        deviation = self.rolling_avg[-1] - self.zero_reference
        self.update_gauge(deviation)

        mean = self.rolling_avg[-1]

        self.mean_gauge_label.setText(
            f"1 Minute Rolling Average: {mean:.2f} mG"
        )
        



    def update_gauge(self, deviation):
        """Update the deviation gauge label."""
        deviation_clamped = max(min(deviation, 3), -3)  # Clamp to ±3
        self.gauge_label.setText(
            f"Deviation from Last Zero: {deviation_clamped:.2f} mG"
        )
        if abs(deviation_clamped) < 1:
            self.gauge_label.setStyleSheet("font-size: 18px; color: green;")
        elif abs(deviation_clamped) < 2:
            self.gauge_label.setStyleSheet("font-size: 18px; color: orange;")
        else:
            self.gauge_label.setStyleSheet("font-size: 18px; color: red;")
    
    def update_label2(self, message):
        self.label2.setText(f"Current Local Time From Rust: {message}")

if __name__ == "__main__":
    app = QApplication(sys.argv)
    gui = DataLoggerGUI()
    gui.show()
    sys.exit(app.exec_())
