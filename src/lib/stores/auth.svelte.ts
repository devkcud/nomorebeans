import type { GetProfileResponse } from '$lib/api/types/profile';

const STORAGE_KEY = 'nomorebeans_current_profile';

class AuthStore {
	currentProfile = $state<GetProfileResponse | null>(null);

	constructor() {
		if (typeof window !== 'undefined') {
			const stored = localStorage.getItem(STORAGE_KEY);
			if (stored) {
				try {
					this.currentProfile = JSON.parse(stored);
				} catch {
					localStorage.removeItem(STORAGE_KEY);
				}
			}
		}
	}

	login(profile: GetProfileResponse) {
		this.currentProfile = profile;
		if (typeof window !== 'undefined') {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(profile));
		}
	}

	logout() {
		this.currentProfile = null;
		if (typeof window !== 'undefined') {
			localStorage.removeItem(STORAGE_KEY);
		}
	}

	get isLoggedIn() {
		return this.currentProfile !== null;
	}
}

export const authStore = new AuthStore();
