module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
        
    },
    theme: {
        colors: {
            'raven': '#d7ffcd',
            white: "#fff"
        },
        fontSize: {
            base: '23px',
            xl: '63px',
	    'main': '36px'
        }
    },
    variants: {},
    plugins: [],
};
