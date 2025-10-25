# Leptos SVG
This crate is a procedural macro that allows for svgs to be embedded from
their source files. This allows for easier maintenance of shared SVGs, and 
declutters the code of components using them while still providing fast 
load times and a more complete SSR (server side rendering) to be done before 
CSR (client side rendering) takes over (eliminates svg 'pop-in' on load).

large svgs or svgs that are used as single use images should be put directly
in an image tag or another element to not bog down the initial load time of the
page.