export const CreateStatusLabel = () => {
    const label = document.querySelector('.status');

    return {
        setStatus: (status) => {
            label.innerText = status;
            label.className = 'status';
            if (status === Status.STARTED)
                return;

            label.classList.add(status.toLowerCase());
        }
    };
}

export const CreateCounter = () => {
    const label = document.querySelector('#counter-display');

    return {
        set: (tick, count) => {
            label.innerText = `t: ${tick} | n: ${count}`;
        }
    };
}

export const Status = Object.freeze({
    STARTED: 'Started',
    RUNNING: 'Running',
    PAUSED: 'Paused',
    STOPPED: 'Stopped'
});
