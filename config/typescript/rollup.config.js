import pkg from './package.json';

export default [
	{
		input: './.ts-output/lib.js',
		output: [
			{ file: pkg.browser, format: 'umd', name: 'config' },
			{ file: pkg.main, format: 'cjs' },
			{ file: pkg.module, format: 'es' }
		]
	}
];
