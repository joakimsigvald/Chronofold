export const CreateSocket = ({ onMessage, onConnect, onPause, onResume, onDisconnect }) => {
    const timeMs = 200;

    let ws = null;
    let isPaused = false;
    let syncCount = 0;
    let timeoutId = null;

    const sendAck = () => ws && ws.readyState === 1 && ws.send("ACK");
    const advanceSync = () => ++syncCount > 1 && !isPaused && sendAck();

    return {
        connect: () => {
            ws = new WebSocket(`ws://${window.location.host}/ws`);

            ws.onopen = () => {
                isPaused = false;
                syncCount = 0;
                onConnect?.();
            };

            ws.onmessage = (event) => {
                syncCount = 0;
                timeoutId = setTimeout(advanceSync, timeMs);
                onMessage?.(event.data);
                advanceSync();
            };

            ws.onclose = () => {
                if (timeoutId) clearTimeout(timeoutId);
                onDisconnect?.();
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
                onPause?.();
                return;
            }

            onResume?.();
            if (syncCount > 1) {
                sendAck();
            }
        }
    };
};