let _slugData = 0;
const load = async ({ params, url }) => {
	const data = params;
	const token = data.slug;

	_slugData = {
		token
	};
};

export { _slugData, load };
