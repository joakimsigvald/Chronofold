export const CreateSocket = ({ onMessage, onConnect, onPause, onResume, onDisconnect }) => {
    let ws = null;
    let isPaused = false;
    let pendingAck = false;

    return {
        connect: () => {
            ws = new WebSocket(`ws://${window.location.host}/ws`);

            ws.onopen = () => {
                isPaused = false;
                pendingAck = false;
                if (onConnect) 
                    onConnect();
            };

            ws.onmessage = (event) => {
                if (onMessage) 
                    onMessage(event.data);

                if (isPaused) {
                    pendingAck = true;
                } else if (ws.readyState === 1) {
                    ws.send("ACK");
                }
            };

            ws.onclose = () => {
                if (onDisconnect) onDisconnect();
            };

            ws.onerror = (err) => console.error("Network Failure:", err);
        },

        disconnect: () => {
            if (ws) {
                ws.onmessage = null;
                ws.close();
            }
        },

        togglePause: () => {
            if (!ws || ws.readyState !== 1) 
                return;

            isPaused = !isPaused;
            if (isPaused) {
                if (onPause) 
                    onPause();
                return;
            }

            if (onResume) 
                onResume();
            if (pendingAck) {
                ws.send("ACK");
                pendingAck = false;
            }
        }
    };
};