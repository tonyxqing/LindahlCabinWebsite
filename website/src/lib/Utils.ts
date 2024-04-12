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