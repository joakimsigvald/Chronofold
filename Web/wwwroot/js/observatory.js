import { ChronofoldApi } from './api.js';
import { Scale } from './config.js';
import { CreateUniverse } from './universe.js';
import { CreatePhysics } from './physics.js';

const Observatory = {
    vacuum: null,
    universe: null,
    physics: null,

    async init() {
        console.log("Chronofold Observatory: Online");
        try {
            this.vacuum = await ChronofoldApi.getVacuum();
            this.physics = CreatePhysics(this.vacuum);
            this.universe = CreateUniverse(this.vacuum, true);
            this.universe.render();
            this.onResize();
            window.addEventListener("resize", () => this.onResize());
            console.log("Starting Heartbeat.");
            this.startHeartbeat(400);
        } catch (error) {
            console.error("Initialization failure:", error);
        }
    },

    onResize() {
        this.universe.scale(window.innerWidth / Scale.resolution);
    },

    startHeartbeat(interval = 1000) {
        setInterval(() => {
            this.physics.advance();
            this.universe.update();
        }, interval);
    }
};

Observatory.init();