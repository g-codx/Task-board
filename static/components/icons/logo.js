const Logo = ({...props}) => {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 512 512"
            width="512"
            height="512"
        >
            <circle cx="256" cy="256" r="256" fill="#ffffff" fill-opacity="1"></circle>
            <g className="" transform="translate(10,0)">
                <path
                    d="M122.31 84.615l-2.85 8.54-11.394 34.185-5.703-5.703L96 115.27 83.27 128l6.367 6.363 26.297 26.297 20.605-61.814 2.845-8.537-17.076-5.695zM151 119v18h242v-18H151zm0 64v18h242v-18H151zm0 64v18h242v-18H151zm-28.69 29.615l-2.85 8.54-11.394 34.185-5.703-5.703L96 307.27 83.27 320l6.367 6.363 26.297 26.297 20.605-61.814 2.845-8.537-17.076-5.695zM151 311v18h242v-18H151zm0 64v18h242v-18H151z"
                    fill="#000000" fill-opacity="1"
                    transform="translate(0, 0) scale(1, 1) rotate(360, 256, 256) skewX(0) skewY(0)"></path>
            </g>
        </svg>
    )
}

export default Logo;