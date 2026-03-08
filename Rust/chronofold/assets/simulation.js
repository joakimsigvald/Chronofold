const display = document.getElementById('tick-display');
        // window.location.host automatically handles 127.0.0.1:3000
        const socket = new WebSocket(`ws://${window.location.host}/ws`);
        const statusLabel = document.querySelector('.status');

        window.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                // 1. Sever the connection
                socket.close();

                // 2. Return to Genesis (Landing Page)
                window.location.href = '/';
            }
        });

        socket.onmessage = (event) => {
            // 1. Update the UI with the incoming Monad state (tick)
            display.innerText = event.data;

            // 2. Send the Internal Handshake (ACK) to trigger the next update cycle
            socket.send("ACK");
        };

        socket.onopen = () => {
            statusLabel.innerText = "Running";
            statusLabel.classList.remove('halted');
            statusLabel.classList.add('running');
        };

        socket.onclose = () => {
            statusLabel.innerText = "Halted";
            statusLabel.classList.remove('running');
            statusLabel.classList.add('halted');
        };

        socket.onerror = (err) => console.error("Genesis Failure:", err);