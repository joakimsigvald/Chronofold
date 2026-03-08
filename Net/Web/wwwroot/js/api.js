export const ChronofoldApi = {
    async getVacuum() {
        const response = await fetch('/api/vacuum');
        if (!response.ok)
            throw new Error("Vacuum retrieval failed");

        return await response.json();
    }
};