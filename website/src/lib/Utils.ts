type NestedObject = {
    [key: string]: unknown
}
export function isEmpty(obj: NestedObject) {
    for (const prop in obj) {
        if (Object.hasOwn(obj, prop)) {
            return false;
        }
    }
    return true;
}
export function computeElapsedString(now: Date, past: Date) {
	const diffInSeconds = Math.floor((now.valueOf() - past.valueOf()) / 1000);

	const units = [
		{ name: 'year', seconds: 31536000 },
		{ name: 'month', seconds: 2592000 },
		{ name: 'week', seconds: 604800 },
		{ name: 'day', seconds: 86400 },
		{ name: 'hour', seconds: 3600 },
		{ name: 'minute', seconds: 60 },
		{ name: 'second', seconds: 1 }
	];

	for (const unit of units) {
		const interval = Math.floor(diffInSeconds / unit.seconds);
		if (interval >= 1) {
			return `${interval} ${unit.name}${interval !== 1 ? 's' : ''} ago`;
		}
	}

	return 'just now';
}

export function parseJwt(token: string) {
		var base64Url = token.split('.')[1];
		var base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
		var jsonPayload = decodeURIComponent(
			window
				.atob(base64)
				.split('')
				.map(function (c) {
					return '%' + ('037777777776' + c.charCodeAt(0).toString(16)).slice(-2);
				})
				.join('')
		);

		return JSON.parse(jsonPayload);
	}