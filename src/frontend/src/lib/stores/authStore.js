import { browser } from '$app/environment';
import { writable } from 'svelte/store';
import { MAX_TIME_LOGIN, DOMAIN, MAX_SYNC_WAIT } from '../code/constants.js';
import { getSync } from '../code/auth.js';

export const authTrigger = writable(0);

const initAuthLocalStorage = () => {
	if (!browser)
		return {
			init: () => {},
			set: (status, usr, authTM) => {},
			check: () => {},
			read: () => {}
		};
	return {
		init: async () => {
			let sV = JSON.parse(localStorage.getItem('authStore')) ?? null;
			let syncKey = 'anon-user'; //await getSync(false,0);
			if (sV == null || sV == 'null') {
				let d = new Date();
				let time = d.getTime() / 1000; // current in secs.
				localStorage.setItem(
					'authStore',
					JSON.stringify({
						data: { loggedIn: 'false', user: syncKey, authTime: time, shortAC: 'abc123' }
					})
				);
				localStorage.setItem('syncStore', JSON.stringify({ data: { lastCheck: 0 } }));
			} else {
				// check expired
				let d = new Date();
				let time = d.getTime() / 1000; // current in secs.
				let sVTime = sV.data.authTime;
				let timeSince = time - sVTime;
				if (timeSince >= MAX_TIME_LOGIN || (sV.data.loggedIn == false && sV.data.user != syncKey)) {
					let d = new Date();
					let time = d.getTime() / 1000; // current in secs.
					localStorage.setItem(
						'authStore',
						JSON.stringify({
							data: { loggedIn: 'false', user: syncKey, authTime: time, shortAC: 'abc123' }
						})
					);
					localStorage.setItem('syncStore', JSON.stringify({ data: { lastCheck: 0 } }));
				}
			}
		},
		set: (status, usr, authTM, shortAC) => {
			if (browser) {
				let data = {
					data: {
						loggedIn: status,
						user: usr,
						authTime: authTM,
						shortAC: shortAC
					}
				};
				localStorage.setItem('authStore', JSON.stringify(data));
				localStorage.setItem('syncStore', JSON.stringify({ data: { lastCheck: authTM } }));
			}
		},
		check: async () => {
			if (browser) {
				let localAuthStore = JSON.parse(localStorage.getItem('authStore')) ?? null;
				let localSyncStore = JSON.parse(localStorage.getItem('syncStore')) ?? null;
				let d = new Date();
				let time = d.getTime() / 1000; // secs
				let since;
				if (localSyncStore != null) {
					since = time - localSyncStore.data.authTime;
				} else {
					since = 0;
				}

				if (localAuthStore != null && localSyncStore != null) {
					let LI = localAuthStore.data.loggedIn;
					// check and update if anon
					if (LI == false || LI == 'false') {
						if (localAuthStore.data.user != 'anon-user') {
							let data2 = {
								data: {
									loggedIn: false,
									user: 'anon-user',
									authTime: time,
									shortAC: 'abc123'
								}
							};
							localStorage.setItem('authStore', JSON.stringify(data2));
							localStorage.setItem('syncStore', JSON.stringify({ data: { lastCheck: time } }));
							return false;
						} else {
							localStorage.setItem('syncStore', JSON.stringify({ data: { lastCheck: time } }));
							return false;
						}
					}

					if (LI == true || LI == 'true') {
						if (since > MAX_TIME_LOGIN) {
							alert('Login Expired - account logged out');
							let data2 = {
								data: {
									loggedIn: false,
									user: 'anon-user',
									authTime: time,
									shortAC: 'abc123'
								}
							};
							localStorage.setItem('authStore', JSON.stringify(data2));
							localStorage.setItem('syncStore', JSON.stringify({ data: { lastCheck: time } }));
							return false;
						} else {
							localStorage.setItem('syncStore', JSON.stringify({ data: { lastCheck: time } }));
							return true;
						}
					}
				}
			}
		},
		read: () => {
			if (browser) {
				return JSON.parse(localStorage.getItem('authStore'));
			}
		}
	};
};

export const authStore = initAuthLocalStorage();
