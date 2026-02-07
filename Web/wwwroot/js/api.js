const ChronofoldApi = {
    async getVersion() {
        const response = await fetch('/api/vacuum/version');
        return await response.text();
    },

    async getVacuum() {
        const response = await fetch('/api/vacuum');
        return await response.json();
    }
};