import { json } from '@sveltejs/kit';
import tokenData from '../../../lib/data/tokens.json';

async function GET(request) {
	// let id = request.url.searchParams.get('id');
	// let key = request.url.searchParams.get('key');
	return json(tokenData);
}
export {GET};
