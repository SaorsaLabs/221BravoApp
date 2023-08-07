let _slugData = 0;
const load = async ({ params, url }) => {
	const data = params;
	const token = data.slug;
	//const title = params.title; for static non-slug?
	const id = url.searchParams.get('id');
	let sub = url?.searchParams?.get('sub');
	if (sub == null || sub == '')
		sub = '0000000000000000000000000000000000000000000000000000000000000000';
	_slugData = {
		token,
		id,
		sub
	};

	// console.log("Slug : ", data);
	// console.log("ID : ", id);
	// console.log("X : ", x);
	//<a href="/page/{href}?id={id}">
	// let res = {
	//   token,
	//   id
	// };
};

export { _slugData, load };
