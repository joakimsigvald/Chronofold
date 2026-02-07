const ChronofoldApi = {
    async getVersion() {
        const response = await fetch('/api/vacuum/version');
        return await response.text();
    },

    async getMonads() {
        const response = await fetch('/api/vacuum/monads');
        const data = await response.json();
        return data.monads;
    }
};