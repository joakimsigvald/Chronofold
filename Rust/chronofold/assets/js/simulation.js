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
            onMessage: (data) => this.counter.set(data),
            onConnect: () => this.statusLabel.setStatus(Status.RUNNING),
            onPause: () => this.statusLabel.setStatus(Status.PAUSED),
            onResume: () => this.statusLabel.setStatus(Status.RUNNING),
            onDisconnect: () => this.statusLabel.setStatus(Status.STOPPED)
        });

        console.log("Chronofold Observatory: Online");
        try {
            this.physics = CreatePhysics(this.vacuum);
            this.universe = CreateUniverse(this.vacuum, true);
            this.universe.render();
            this.setupControls();
            this.socket.connect();
        } catch (error) {
            console.error("Initialization failure:", error);
        }
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