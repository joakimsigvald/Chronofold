const Observatory = {
    display: null,
    statusLabel: null,
    socket: null,

    init() {
        this.display = document.getElementById('tick-display');
        this.statusLabel = document.querySelector('.status');

        console.log("Chronofold Observatory: Online");
        this.setupControls();
        this.connect();
    },

    setupControls() {
        window.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                if (this.socket)
                    this.socket.close();

                window.location.href = '/';
            }
        });
    },

    connect() {
        // window.location.host automatically handles 127.0.0.1:3000
        this.socket = new WebSocket(`ws://${window.location.host}/ws`);

        this.socket.onopen = () => {
            this.statusLabel.innerText = "Running";
            this.statusLabel.classList.remove('halted');
            this.statusLabel.classList.add('running');
        };

        this.socket.onmessage = (event) => {
            // 1. Update the UI with the incoming Monad state (tick)
            this.display.innerText = event.data;

            // 2. Send the Internal Handshake (ACK) to trigger the next update cycle
            this.socket.send("ACK");
        };

        this.socket.onclose = () => {
            this.statusLabel.innerText = "Halted";
            this.statusLabel.classList.remove('running');
            this.statusLabel.classList.add('halted');
        };

        this.socket.onerror = (err) => console.error("Genesis Failure:", err);
    }
}

Observatory.init();
