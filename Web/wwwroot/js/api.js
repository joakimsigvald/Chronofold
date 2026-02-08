const ChronofoldApi = {
    async getVacuum() {
        const response = await fetch('/api/vacuum');
        return await response.json();
    }
};