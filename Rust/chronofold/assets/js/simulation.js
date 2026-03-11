import { Scale } from './config.js';
import { CreateUniverse } from './universe.js';
import { CreatePhysics } from './physics.js';
import { CreateStatusLabel, Status, CreateCounter } from './header.js';
import { CreateSocket } from './socket.js';

const Simulation = {
    counter: null,
    statusLabel: null,
    socket: null,
    vacuum: { monads: [], links: [] },
    universe: null,
    physics: null,
    isPaused: false,

    async init() {
        this.counter = CreateCounter();
        this.statusLabel = CreateStatusLabel();

        this.socket = CreateSocket({
            onMessage: (data) => this.receive(data),
            onConnect: () => this.statusLabel.setStatus(Status.RUNNING),
            onPause: () => this.statusLabel.setStatus(Status.PAUSED),
            onResume: () => this.statusLabel.setStatus(Status.RUNNING),
            onDisconnect: () => this.statusLabel.setStatus(Status.STOPPED)
        });

        console.log("Chronofold Observatory: Online");
        try {
            this.physics = CreatePhysics(this.vacuum);
            this.universe = CreateUniverse(this.vacuum);
            this.universe.init();
            this.setupControls();
            this.socket.connect();
        } catch (error) {
            console.error("Initialization failure:", error);
        }
    },

    merge(oldMonads, newMonads) {
        const monadMap = new Map(oldMonads.map(m => [m.id, m]));
        return newMonads.map(m => ({ ...monadMap.get(m.id), ...m })); 
    },

    receive(data) {
        const state = JSON.parse(data);
        this.vacuum.monads = this.merge(this.vacuum.monads, state.monads);
        this.vacuum.links = state.links;
        this.counter.set(state.tick);
        this.universe.update();
    },

    setupControls() {
        this.onResize();
        window.addEventListener("resize", () => this.onResize());

        window.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                this.socket.disconnect();

                setTimeout(() => {
                    window.location.assign('/');
                }, 100);
            }
            if (e.key === 'Enter') {
                this.socket.togglePause();
            }
        });
    },

    onResize() {
        this.universe.scale(window.innerWidth / Scale.resolution);
    },
}

Simulation.init();