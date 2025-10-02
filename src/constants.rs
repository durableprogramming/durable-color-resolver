use std::collections::HashMap;

pub fn color_constants() -> HashMap<String, String> {
    let mut colors = HashMap::new();

    // Tailwind colors in OKLCH
    // Red
    colors.insert("red-50".to_string(), "oklch(0.971 0.013 17.38)".to_string());
    colors.insert(
        "red-100".to_string(),
        "oklch(0.936 0.032 17.717)".to_string(),
    );
    colors.insert(
        "red-200".to_string(),
        "oklch(0.885 0.062 18.334)".to_string(),
    );
    colors.insert(
        "red-300".to_string(),
        "oklch(0.808 0.114 19.571)".to_string(),
    );
    colors.insert(
        "red-400".to_string(),
        "oklch(0.704 0.191 22.216)".to_string(),
    );
    colors.insert(
        "red-500".to_string(),
        "oklch(0.637 0.237 25.331)".to_string(),
    );
    colors.insert(
        "red-600".to_string(),
        "oklch(0.577 0.245 27.325)".to_string(),
    );
    colors.insert(
        "red-700".to_string(),
        "oklch(0.505 0.213 27.518)".to_string(),
    );
    colors.insert(
        "red-800".to_string(),
        "oklch(0.444 0.177 26.899)".to_string(),
    );
    colors.insert(
        "red-900".to_string(),
        "oklch(0.396 0.141 25.723)".to_string(),
    );
    colors.insert(
        "red-950".to_string(),
        "oklch(0.258 0.092 26.042)".to_string(),
    );

    // Orange
    colors.insert(
        "orange-50".to_string(),
        "oklch(0.98 0.016 73.684)".to_string(),
    );
    colors.insert(
        "orange-100".to_string(),
        "oklch(0.954 0.038 75.164)".to_string(),
    );
    colors.insert(
        "orange-200".to_string(),
        "oklch(0.901 0.076 70.697)".to_string(),
    );
    colors.insert(
        "orange-300".to_string(),
        "oklch(0.837 0.128 66.29)".to_string(),
    );
    colors.insert(
        "orange-400".to_string(),
        "oklch(0.75 0.183 55.934)".to_string(),
    );
    colors.insert(
        "orange-500".to_string(),
        "oklch(0.705 0.213 47.604)".to_string(),
    );
    colors.insert(
        "orange-600".to_string(),
        "oklch(0.646 0.222 41.116)".to_string(),
    );
    colors.insert(
        "orange-700".to_string(),
        "oklch(0.553 0.195 38.402)".to_string(),
    );
    colors.insert(
        "orange-800".to_string(),
        "oklch(0.47 0.157 37.304)".to_string(),
    );
    colors.insert(
        "orange-900".to_string(),
        "oklch(0.408 0.123 38.172)".to_string(),
    );
    colors.insert(
        "orange-950".to_string(),
        "oklch(0.266 0.079 36.259)".to_string(),
    );

    // Amber
    colors.insert(
        "amber-50".to_string(),
        "oklch(0.987 0.022 95.277)".to_string(),
    );
    colors.insert(
        "amber-100".to_string(),
        "oklch(0.962 0.059 95.617)".to_string(),
    );
    colors.insert(
        "amber-200".to_string(),
        "oklch(0.924 0.12 95.746)".to_string(),
    );
    colors.insert(
        "amber-300".to_string(),
        "oklch(0.879 0.169 91.605)".to_string(),
    );
    colors.insert(
        "amber-400".to_string(),
        "oklch(0.828 0.189 84.429)".to_string(),
    );
    colors.insert(
        "amber-500".to_string(),
        "oklch(0.769 0.188 70.08)".to_string(),
    );
    colors.insert(
        "amber-600".to_string(),
        "oklch(0.666 0.179 58.318)".to_string(),
    );
    colors.insert(
        "amber-700".to_string(),
        "oklch(0.555 0.163 48.998)".to_string(),
    );
    colors.insert(
        "amber-800".to_string(),
        "oklch(0.473 0.137 46.201)".to_string(),
    );
    colors.insert(
        "amber-900".to_string(),
        "oklch(0.414 0.112 45.904)".to_string(),
    );
    colors.insert(
        "amber-950".to_string(),
        "oklch(0.279 0.077 45.635)".to_string(),
    );

    // Yellow
    colors.insert(
        "yellow-50".to_string(),
        "oklch(0.987 0.026 102.212)".to_string(),
    );
    colors.insert(
        "yellow-100".to_string(),
        "oklch(0.973 0.071 103.193)".to_string(),
    );
    colors.insert(
        "yellow-200".to_string(),
        "oklch(0.945 0.129 101.54)".to_string(),
    );
    colors.insert(
        "yellow-300".to_string(),
        "oklch(0.905 0.182 98.111)".to_string(),
    );
    colors.insert(
        "yellow-400".to_string(),
        "oklch(0.852 0.199 91.936)".to_string(),
    );
    colors.insert(
        "yellow-500".to_string(),
        "oklch(0.795 0.184 86.047)".to_string(),
    );
    colors.insert(
        "yellow-600".to_string(),
        "oklch(0.681 0.162 75.834)".to_string(),
    );
    colors.insert(
        "yellow-700".to_string(),
        "oklch(0.554 0.135 66.442)".to_string(),
    );
    colors.insert(
        "yellow-800".to_string(),
        "oklch(0.476 0.114 61.907)".to_string(),
    );
    colors.insert(
        "yellow-900".to_string(),
        "oklch(0.421 0.095 57.708)".to_string(),
    );
    colors.insert(
        "yellow-950".to_string(),
        "oklch(0.286 0.066 53.813)".to_string(),
    );

    // Lime
    colors.insert(
        "lime-50".to_string(),
        "oklch(0.986 0.031 120.757)".to_string(),
    );
    colors.insert(
        "lime-100".to_string(),
        "oklch(0.967 0.067 122.328)".to_string(),
    );
    colors.insert(
        "lime-200".to_string(),
        "oklch(0.938 0.127 124.321)".to_string(),
    );
    colors.insert(
        "lime-300".to_string(),
        "oklch(0.897 0.196 126.665)".to_string(),
    );
    colors.insert(
        "lime-400".to_string(),
        "oklch(0.841 0.238 128.85)".to_string(),
    );
    colors.insert(
        "lime-500".to_string(),
        "oklch(0.768 0.233 130.85)".to_string(),
    );
    colors.insert(
        "lime-600".to_string(),
        "oklch(0.648 0.2 131.684)".to_string(),
    );
    colors.insert(
        "lime-700".to_string(),
        "oklch(0.532 0.157 131.589)".to_string(),
    );
    colors.insert(
        "lime-800".to_string(),
        "oklch(0.453 0.124 130.933)".to_string(),
    );
    colors.insert(
        "lime-900".to_string(),
        "oklch(0.405 0.101 131.063)".to_string(),
    );
    colors.insert(
        "lime-950".to_string(),
        "oklch(0.274 0.072 132.109)".to_string(),
    );

    // Green
    colors.insert(
        "green-50".to_string(),
        "oklch(0.982 0.018 155.826)".to_string(),
    );
    colors.insert(
        "green-100".to_string(),
        "oklch(0.962 0.044 156.743)".to_string(),
    );
    colors.insert(
        "green-200".to_string(),
        "oklch(0.925 0.084 155.995)".to_string(),
    );
    colors.insert(
        "green-300".to_string(),
        "oklch(0.871 0.15 154.449)".to_string(),
    );
    colors.insert(
        "green-400".to_string(),
        "oklch(0.792 0.209 151.711)".to_string(),
    );
    colors.insert(
        "green-500".to_string(),
        "oklch(0.723 0.219 149.579)".to_string(),
    );
    colors.insert(
        "green-600".to_string(),
        "oklch(0.627 0.194 149.214)".to_string(),
    );
    colors.insert(
        "green-700".to_string(),
        "oklch(0.527 0.154 150.069)".to_string(),
    );
    colors.insert(
        "green-800".to_string(),
        "oklch(0.448 0.119 151.328)".to_string(),
    );
    colors.insert(
        "green-900".to_string(),
        "oklch(0.393 0.095 152.535)".to_string(),
    );
    colors.insert(
        "green-950".to_string(),
        "oklch(0.266 0.065 152.934)".to_string(),
    );

    // Emerald
    colors.insert(
        "emerald-50".to_string(),
        "oklch(0.979 0.021 166.113)".to_string(),
    );
    colors.insert(
        "emerald-100".to_string(),
        "oklch(0.95 0.052 163.051)".to_string(),
    );
    colors.insert(
        "emerald-200".to_string(),
        "oklch(0.905 0.093 164.15)".to_string(),
    );
    colors.insert(
        "emerald-300".to_string(),
        "oklch(0.845 0.143 164.978)".to_string(),
    );
    colors.insert(
        "emerald-400".to_string(),
        "oklch(0.765 0.177 163.223)".to_string(),
    );
    colors.insert(
        "emerald-500".to_string(),
        "oklch(0.696 0.17 162.48)".to_string(),
    );
    colors.insert(
        "emerald-600".to_string(),
        "oklch(0.596 0.145 163.225)".to_string(),
    );
    colors.insert(
        "emerald-700".to_string(),
        "oklch(0.508 0.118 165.612)".to_string(),
    );
    colors.insert(
        "emerald-800".to_string(),
        "oklch(0.432 0.095 166.913)".to_string(),
    );
    colors.insert(
        "emerald-900".to_string(),
        "oklch(0.378 0.077 168.94)".to_string(),
    );
    colors.insert(
        "emerald-950".to_string(),
        "oklch(0.262 0.051 172.552)".to_string(),
    );

    // Teal
    colors.insert(
        "teal-50".to_string(),
        "oklch(0.984 0.014 180.72)".to_string(),
    );
    colors.insert(
        "teal-100".to_string(),
        "oklch(0.953 0.051 180.801)".to_string(),
    );
    colors.insert(
        "teal-200".to_string(),
        "oklch(0.91 0.096 180.426)".to_string(),
    );
    colors.insert(
        "teal-300".to_string(),
        "oklch(0.855 0.138 181.071)".to_string(),
    );
    colors.insert(
        "teal-400".to_string(),
        "oklch(0.777 0.152 181.912)".to_string(),
    );
    colors.insert(
        "teal-500".to_string(),
        "oklch(0.704 0.14 182.503)".to_string(),
    );
    colors.insert(
        "teal-600".to_string(),
        "oklch(0.6 0.118 184.704)".to_string(),
    );
    colors.insert(
        "teal-700".to_string(),
        "oklch(0.511 0.096 186.391)".to_string(),
    );
    colors.insert(
        "teal-800".to_string(),
        "oklch(0.437 0.078 188.216)".to_string(),
    );
    colors.insert(
        "teal-900".to_string(),
        "oklch(0.386 0.063 188.416)".to_string(),
    );
    colors.insert(
        "teal-950".to_string(),
        "oklch(0.277 0.046 192.524)".to_string(),
    );

    // Cyan
    colors.insert(
        "cyan-50".to_string(),
        "oklch(0.984 0.019 200.873)".to_string(),
    );
    colors.insert(
        "cyan-100".to_string(),
        "oklch(0.956 0.045 203.388)".to_string(),
    );
    colors.insert(
        "cyan-200".to_string(),
        "oklch(0.917 0.08 205.041)".to_string(),
    );
    colors.insert(
        "cyan-300".to_string(),
        "oklch(0.865 0.127 207.078)".to_string(),
    );
    colors.insert(
        "cyan-400".to_string(),
        "oklch(0.789 0.154 211.53)".to_string(),
    );
    colors.insert(
        "cyan-500".to_string(),
        "oklch(0.715 0.143 215.221)".to_string(),
    );
    colors.insert(
        "cyan-600".to_string(),
        "oklch(0.609 0.126 221.723)".to_string(),
    );
    colors.insert(
        "cyan-700".to_string(),
        "oklch(0.52 0.105 223.128)".to_string(),
    );
    colors.insert(
        "cyan-800".to_string(),
        "oklch(0.45 0.085 224.283)".to_string(),
    );
    colors.insert(
        "cyan-900".to_string(),
        "oklch(0.398 0.07 227.392)".to_string(),
    );
    colors.insert(
        "cyan-950".to_string(),
        "oklch(0.302 0.056 229.695)".to_string(),
    );

    // Sky
    colors.insert(
        "sky-50".to_string(),
        "oklch(0.977 0.013 236.62)".to_string(),
    );
    colors.insert(
        "sky-100".to_string(),
        "oklch(0.951 0.026 236.824)".to_string(),
    );
    colors.insert(
        "sky-200".to_string(),
        "oklch(0.901 0.058 230.902)".to_string(),
    );
    colors.insert(
        "sky-300".to_string(),
        "oklch(0.828 0.111 230.318)".to_string(),
    );
    colors.insert(
        "sky-400".to_string(),
        "oklch(0.746 0.16 232.661)".to_string(),
    );
    colors.insert(
        "sky-500".to_string(),
        "oklch(0.685 0.169 237.323)".to_string(),
    );
    colors.insert(
        "sky-600".to_string(),
        "oklch(0.588 0.158 241.966)".to_string(),
    );
    colors.insert(
        "sky-700".to_string(),
        "oklch(0.5 0.134 242.749)".to_string(),
    );
    colors.insert(
        "sky-800".to_string(),
        "oklch(0.443 0.11 240.79)".to_string(),
    );
    colors.insert(
        "sky-900".to_string(),
        "oklch(0.391 0.09 240.876)".to_string(),
    );
    colors.insert(
        "sky-950".to_string(),
        "oklch(0.293 0.066 243.157)".to_string(),
    );

    // Blue
    colors.insert(
        "blue-50".to_string(),
        "oklch(0.97 0.014 254.604)".to_string(),
    );
    colors.insert(
        "blue-100".to_string(),
        "oklch(0.932 0.032 255.585)".to_string(),
    );
    colors.insert(
        "blue-200".to_string(),
        "oklch(0.882 0.059 254.128)".to_string(),
    );
    colors.insert(
        "blue-300".to_string(),
        "oklch(0.809 0.105 251.813)".to_string(),
    );
    colors.insert(
        "blue-400".to_string(),
        "oklch(0.707 0.165 254.624)".to_string(),
    );
    colors.insert(
        "blue-500".to_string(),
        "oklch(0.623 0.214 259.815)".to_string(),
    );
    colors.insert(
        "blue-600".to_string(),
        "oklch(0.546 0.245 262.881)".to_string(),
    );
    colors.insert(
        "blue-700".to_string(),
        "oklch(0.488 0.243 264.376)".to_string(),
    );
    colors.insert(
        "blue-800".to_string(),
        "oklch(0.424 0.199 265.638)".to_string(),
    );
    colors.insert(
        "blue-900".to_string(),
        "oklch(0.379 0.146 265.522)".to_string(),
    );
    colors.insert(
        "blue-950".to_string(),
        "oklch(0.282 0.091 267.935)".to_string(),
    );

    // Indigo
    colors.insert(
        "indigo-50".to_string(),
        "oklch(0.962 0.018 272.314)".to_string(),
    );
    colors.insert(
        "indigo-100".to_string(),
        "oklch(0.93 0.034 272.788)".to_string(),
    );
    colors.insert(
        "indigo-200".to_string(),
        "oklch(0.87 0.065 274.039)".to_string(),
    );
    colors.insert(
        "indigo-300".to_string(),
        "oklch(0.785 0.115 274.713)".to_string(),
    );
    colors.insert(
        "indigo-400".to_string(),
        "oklch(0.673 0.182 276.935)".to_string(),
    );
    colors.insert(
        "indigo-500".to_string(),
        "oklch(0.585 0.233 277.117)".to_string(),
    );
    colors.insert(
        "indigo-600".to_string(),
        "oklch(0.511 0.262 276.966)".to_string(),
    );
    colors.insert(
        "indigo-700".to_string(),
        "oklch(0.457 0.24 277.023)".to_string(),
    );
    colors.insert(
        "indigo-800".to_string(),
        "oklch(0.398 0.195 277.366)".to_string(),
    );
    colors.insert(
        "indigo-900".to_string(),
        "oklch(0.359 0.144 278.697)".to_string(),
    );
    colors.insert(
        "indigo-950".to_string(),
        "oklch(0.257 0.09 281.288)".to_string(),
    );

    // Violet
    colors.insert(
        "violet-50".to_string(),
        "oklch(0.969 0.016 293.756)".to_string(),
    );
    colors.insert(
        "violet-100".to_string(),
        "oklch(0.943 0.029 294.588)".to_string(),
    );
    colors.insert(
        "violet-200".to_string(),
        "oklch(0.894 0.057 293.283)".to_string(),
    );
    colors.insert(
        "violet-300".to_string(),
        "oklch(0.811 0.111 293.571)".to_string(),
    );
    colors.insert(
        "violet-400".to_string(),
        "oklch(0.702 0.183 293.541)".to_string(),
    );
    colors.insert(
        "violet-500".to_string(),
        "oklch(0.606 0.25 292.717)".to_string(),
    );
    colors.insert(
        "violet-600".to_string(),
        "oklch(0.541 0.281 293.009)".to_string(),
    );
    colors.insert(
        "violet-700".to_string(),
        "oklch(0.491 0.27 292.581)".to_string(),
    );
    colors.insert(
        "violet-800".to_string(),
        "oklch(0.432 0.232 292.759)".to_string(),
    );
    colors.insert(
        "violet-900".to_string(),
        "oklch(0.38 0.189 293.745)".to_string(),
    );
    colors.insert(
        "violet-950".to_string(),
        "oklch(0.283 0.141 291.089)".to_string(),
    );

    // Purple
    colors.insert(
        "purple-50".to_string(),
        "oklch(0.977 0.014 308.299)".to_string(),
    );
    colors.insert(
        "purple-100".to_string(),
        "oklch(0.946 0.033 307.174)".to_string(),
    );
    colors.insert(
        "purple-200".to_string(),
        "oklch(0.902 0.063 306.703)".to_string(),
    );
    colors.insert(
        "purple-300".to_string(),
        "oklch(0.827 0.119 306.383)".to_string(),
    );
    colors.insert(
        "purple-400".to_string(),
        "oklch(0.714 0.203 305.504)".to_string(),
    );
    colors.insert(
        "purple-500".to_string(),
        "oklch(0.627 0.265 303.9)".to_string(),
    );
    colors.insert(
        "purple-600".to_string(),
        "oklch(0.558 0.288 302.321)".to_string(),
    );
    colors.insert(
        "purple-700".to_string(),
        "oklch(0.496 0.265 301.924)".to_string(),
    );
    colors.insert(
        "purple-800".to_string(),
        "oklch(0.438 0.218 303.724)".to_string(),
    );
    colors.insert(
        "purple-900".to_string(),
        "oklch(0.381 0.176 304.987)".to_string(),
    );
    colors.insert(
        "purple-950".to_string(),
        "oklch(0.291 0.149 302.717)".to_string(),
    );

    // Fuchsia
    colors.insert(
        "fuchsia-50".to_string(),
        "oklch(0.977 0.017 320.058)".to_string(),
    );
    colors.insert(
        "fuchsia-100".to_string(),
        "oklch(0.952 0.037 318.852)".to_string(),
    );
    colors.insert(
        "fuchsia-200".to_string(),
        "oklch(0.903 0.076 319.62)".to_string(),
    );
    colors.insert(
        "fuchsia-300".to_string(),
        "oklch(0.833 0.145 321.434)".to_string(),
    );
    colors.insert(
        "fuchsia-400".to_string(),
        "oklch(0.74 0.238 322.16)".to_string(),
    );
    colors.insert(
        "fuchsia-500".to_string(),
        "oklch(0.667 0.295 322.15)".to_string(),
    );
    colors.insert(
        "fuchsia-600".to_string(),
        "oklch(0.591 0.293 322.896)".to_string(),
    );
    colors.insert(
        "fuchsia-700".to_string(),
        "oklch(0.518 0.253 323.949)".to_string(),
    );
    colors.insert(
        "fuchsia-800".to_string(),
        "oklch(0.452 0.211 324.591)".to_string(),
    );
    colors.insert(
        "fuchsia-900".to_string(),
        "oklch(0.401 0.17 325.612)".to_string(),
    );
    colors.insert(
        "fuchsia-950".to_string(),
        "oklch(0.293 0.136 325.661)".to_string(),
    );

    // Pink
    colors.insert(
        "pink-50".to_string(),
        "oklch(0.971 0.014 343.198)".to_string(),
    );
    colors.insert(
        "pink-100".to_string(),
        "oklch(0.948 0.028 342.258)".to_string(),
    );
    colors.insert(
        "pink-200".to_string(),
        "oklch(0.899 0.061 343.231)".to_string(),
    );
    colors.insert(
        "pink-300".to_string(),
        "oklch(0.823 0.12 346.018)".to_string(),
    );
    colors.insert(
        "pink-400".to_string(),
        "oklch(0.718 0.202 349.761)".to_string(),
    );
    colors.insert(
        "pink-500".to_string(),
        "oklch(0.656 0.241 354.308)".to_string(),
    );
    colors.insert(
        "pink-600".to_string(),
        "oklch(0.592 0.249 0.584)".to_string(),
    );
    colors.insert(
        "pink-700".to_string(),
        "oklch(0.525 0.223 3.958)".to_string(),
    );
    colors.insert(
        "pink-800".to_string(),
        "oklch(0.459 0.187 3.815)".to_string(),
    );
    colors.insert(
        "pink-900".to_string(),
        "oklch(0.408 0.153 2.432)".to_string(),
    );
    colors.insert(
        "pink-950".to_string(),
        "oklch(0.284 0.109 3.907)".to_string(),
    );

    // Rose
    colors.insert(
        "rose-50".to_string(),
        "oklch(0.969 0.015 12.422)".to_string(),
    );
    colors.insert(
        "rose-100".to_string(),
        "oklch(0.941 0.03 12.58)".to_string(),
    );
    colors.insert(
        "rose-200".to_string(),
        "oklch(0.892 0.058 10.001)".to_string(),
    );
    colors.insert(
        "rose-300".to_string(),
        "oklch(0.81 0.117 11.638)".to_string(),
    );
    colors.insert(
        "rose-400".to_string(),
        "oklch(0.712 0.194 13.428)".to_string(),
    );
    colors.insert(
        "rose-500".to_string(),
        "oklch(0.645 0.246 16.439)".to_string(),
    );
    colors.insert(
        "rose-600".to_string(),
        "oklch(0.586 0.253 17.585)".to_string(),
    );
    colors.insert(
        "rose-700".to_string(),
        "oklch(0.514 0.222 16.935)".to_string(),
    );
    colors.insert(
        "rose-800".to_string(),
        "oklch(0.455 0.188 13.697)".to_string(),
    );
    colors.insert(
        "rose-900".to_string(),
        "oklch(0.41 0.159 10.272)".to_string(),
    );
    colors.insert(
        "rose-950".to_string(),
        "oklch(0.271 0.105 12.094)".to_string(),
    );

    // Slate
    colors.insert(
        "slate-50".to_string(),
        "oklch(0.984 0.003 247.858)".to_string(),
    );
    colors.insert(
        "slate-100".to_string(),
        "oklch(0.968 0.007 247.896)".to_string(),
    );
    colors.insert(
        "slate-200".to_string(),
        "oklch(0.929 0.013 255.508)".to_string(),
    );
    colors.insert(
        "slate-300".to_string(),
        "oklch(0.869 0.022 252.894)".to_string(),
    );
    colors.insert(
        "slate-400".to_string(),
        "oklch(0.704 0.04 256.788)".to_string(),
    );
    colors.insert(
        "slate-500".to_string(),
        "oklch(0.554 0.046 257.417)".to_string(),
    );
    colors.insert(
        "slate-600".to_string(),
        "oklch(0.446 0.043 257.281)".to_string(),
    );
    colors.insert(
        "slate-700".to_string(),
        "oklch(0.372 0.044 257.287)".to_string(),
    );
    colors.insert(
        "slate-800".to_string(),
        "oklch(0.279 0.041 260.031)".to_string(),
    );
    colors.insert(
        "slate-900".to_string(),
        "oklch(0.208 0.042 265.755)".to_string(),
    );
    colors.insert(
        "slate-950".to_string(),
        "oklch(0.129 0.042 264.695)".to_string(),
    );

    // Gray
    colors.insert(
        "gray-50".to_string(),
        "oklch(0.985 0.002 247.839)".to_string(),
    );
    colors.insert(
        "gray-100".to_string(),
        "oklch(0.967 0.003 264.542)".to_string(),
    );
    colors.insert(
        "gray-200".to_string(),
        "oklch(0.928 0.006 264.531)".to_string(),
    );
    colors.insert(
        "gray-300".to_string(),
        "oklch(0.872 0.01 258.338)".to_string(),
    );
    colors.insert(
        "gray-400".to_string(),
        "oklch(0.707 0.022 261.325)".to_string(),
    );
    colors.insert(
        "gray-500".to_string(),
        "oklch(0.551 0.027 264.364)".to_string(),
    );
    colors.insert(
        "gray-600".to_string(),
        "oklch(0.446 0.03 256.802)".to_string(),
    );
    colors.insert(
        "gray-700".to_string(),
        "oklch(0.373 0.034 259.733)".to_string(),
    );
    colors.insert(
        "gray-800".to_string(),
        "oklch(0.278 0.033 256.848)".to_string(),
    );
    colors.insert(
        "gray-900".to_string(),
        "oklch(0.21 0.034 264.665)".to_string(),
    );
    colors.insert(
        "gray-950".to_string(),
        "oklch(0.13 0.028 261.692)".to_string(),
    );

    // Zinc
    colors.insert("zinc-50".to_string(), "oklch(0.985 0 0)".to_string());
    colors.insert(
        "zinc-100".to_string(),
        "oklch(0.967 0.001 286.375)".to_string(),
    );
    colors.insert(
        "zinc-200".to_string(),
        "oklch(0.92 0.004 286.32)".to_string(),
    );
    colors.insert(
        "zinc-300".to_string(),
        "oklch(0.871 0.006 286.286)".to_string(),
    );
    colors.insert(
        "zinc-400".to_string(),
        "oklch(0.705 0.015 286.067)".to_string(),
    );
    colors.insert(
        "zinc-500".to_string(),
        "oklch(0.552 0.016 285.938)".to_string(),
    );
    colors.insert(
        "zinc-600".to_string(),
        "oklch(0.442 0.017 285.786)".to_string(),
    );
    colors.insert(
        "zinc-700".to_string(),
        "oklch(0.37 0.013 285.805)".to_string(),
    );
    colors.insert(
        "zinc-800".to_string(),
        "oklch(0.274 0.006 286.033)".to_string(),
    );
    colors.insert(
        "zinc-900".to_string(),
        "oklch(0.21 0.006 285.885)".to_string(),
    );
    colors.insert(
        "zinc-950".to_string(),
        "oklch(0.141 0.005 285.823)".to_string(),
    );

    // Neutral
    colors.insert("neutral-50".to_string(), "oklch(0.985 0 0)".to_string());
    colors.insert("neutral-100".to_string(), "oklch(0.97 0 0)".to_string());
    colors.insert("neutral-200".to_string(), "oklch(0.922 0 0)".to_string());
    colors.insert("neutral-300".to_string(), "oklch(0.87 0 0)".to_string());
    colors.insert("neutral-400".to_string(), "oklch(0.708 0 0)".to_string());
    colors.insert("neutral-500".to_string(), "oklch(0.556 0 0)".to_string());
    colors.insert("neutral-600".to_string(), "oklch(0.439 0 0)".to_string());
    colors.insert("neutral-700".to_string(), "oklch(0.371 0 0)".to_string());
    colors.insert("neutral-800".to_string(), "oklch(0.269 0 0)".to_string());
    colors.insert("neutral-900".to_string(), "oklch(0.205 0 0)".to_string());
    colors.insert("neutral-950".to_string(), "oklch(0.145 0 0)".to_string());

    // Stone
    colors.insert(
        "stone-50".to_string(),
        "oklch(0.985 0.001 106.423)".to_string(),
    );
    colors.insert(
        "stone-100".to_string(),
        "oklch(0.97 0.001 106.424)".to_string(),
    );
    colors.insert(
        "stone-200".to_string(),
        "oklch(0.923 0.003 48.717)".to_string(),
    );
    colors.insert(
        "stone-300".to_string(),
        "oklch(0.869 0.005 56.366)".to_string(),
    );
    colors.insert(
        "stone-400".to_string(),
        "oklch(0.709 0.01 56.259)".to_string(),
    );
    colors.insert(
        "stone-500".to_string(),
        "oklch(0.553 0.013 58.071)".to_string(),
    );
    colors.insert(
        "stone-600".to_string(),
        "oklch(0.444 0.011 73.639)".to_string(),
    );
    colors.insert(
        "stone-700".to_string(),
        "oklch(0.374 0.01 67.558)".to_string(),
    );
    colors.insert(
        "stone-800".to_string(),
        "oklch(0.268 0.007 34.298)".to_string(),
    );
    colors.insert(
        "stone-900".to_string(),
        "oklch(0.216 0.006 56.043)".to_string(),
    );
    colors.insert(
        "stone-950".to_string(),
        "oklch(0.147 0.004 49.25)".to_string(),
    );

    // CSS named colors
    colors.insert("red".to_string(), "oklch(0.637 0.237 25.331)".to_string());
    colors.insert(
        "orange".to_string(),
        "oklch(0.705 0.213 47.604)".to_string(),
    );
    colors.insert(
        "yellow".to_string(),
        "oklch(0.795 0.184 86.047)".to_string(),
    );
    colors.insert(
        "green".to_string(),
        "oklch(0.723 0.219 149.579)".to_string(),
    );
    colors.insert("blue".to_string(), "oklch(0.623 0.214 259.815)".to_string());
    colors.insert("purple".to_string(), "oklch(0.627 0.265 303.9)".to_string());
    colors.insert("pink".to_string(), "oklch(0.656 0.241 354.308)".to_string());
    colors.insert("black".to_string(), "#000000".to_string());
    colors.insert("white".to_string(), "#ffffff".to_string());
    colors.insert("gray".to_string(), "oklch(0.551 0.027 264.364)".to_string());
    colors.insert(
        "silver".to_string(),
        "oklch(0.707 0.022 261.325)".to_string(),
    );

    colors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_constants_contains_tailwind_colors() {
        let constants = color_constants();
        // Test some key colors
        assert!(constants.contains_key("red-500"));
        assert!(constants.contains_key("blue-500"));
        assert!(constants.contains_key("green-500"));
        assert!(constants.contains_key("yellow-500"));
        assert!(constants.contains_key("purple-500"));
        assert!(constants.contains_key("pink-500"));
    }

    #[test]
    fn test_color_constants_contains_css_colors() {
        let constants = color_constants();
        assert!(constants.contains_key("red"));
        assert!(constants.contains_key("blue"));
        assert!(constants.contains_key("green"));
        assert!(constants.contains_key("black"));
        assert!(constants.contains_key("white"));
        assert!(constants.contains_key("gray"));
        assert!(constants.contains_key("silver"));
    }

    #[test]
    fn test_color_constants_tailwind_shades() {
        let constants = color_constants();
        // Test that all shades are present for red
        for shade in [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950].iter() {
            let key = format!("red-{}", shade);
            assert!(constants.contains_key(&key), "Missing {}", key);
        }
    }

    #[test]
    fn test_color_constants_oklch_format() {
        let constants = color_constants();
        // Most colors should be in OKLCH format
        let red_500 = constants.get("red-500").unwrap();
        assert!(red_500.starts_with("oklch("));
        assert!(red_500.ends_with(")"));
    }

    #[test]
    fn test_color_constants_hex_format() {
        let constants = color_constants();
        // Black and white are hex
        assert_eq!(constants.get("black").unwrap(), "#000000");
        assert_eq!(constants.get("white").unwrap(), "#ffffff");
    }

    #[test]
    fn test_color_constants_count() {
        let constants = color_constants();
        // Should have many colors: 22 families * 11 shades + css colors
        // 22 families: red, orange, amber, yellow, lime, green, emerald, teal, cyan, sky, blue, indigo, violet, purple, fuchsia, pink, rose, slate, gray, zinc, neutral, stone
        // 11 shades each: 50,100,200,300,400,500,600,700,800,900,950
        // Plus CSS colors: red, orange, yellow, green, blue, purple, pink, black, white, gray, silver
        // Some overlap, but approximately 22*11 + 11 = 253
        assert!(
            constants.len() > 200,
            "Expected >200 colors, got {}",
            constants.len()
        );
    }

    #[test]
    fn test_color_constants_no_duplicates() {
        let constants = color_constants();
        let mut seen = std::collections::HashSet::new();
        for (key, _value) in &constants {
            assert!(!seen.contains(key), "Duplicate key: {}", key);
            seen.insert(key.clone());
            // Also check no duplicate values (though possible)
        }
    }

    #[test]
    fn test_color_constants_valid_oklch() {
        let constants = color_constants();
        for (key, value) in &constants {
            if value.starts_with("oklch(") {
                // Should be parseable
                assert!(
                    value.ends_with(")"),
                    "Invalid OKLCH format for {}: {}",
                    key,
                    value
                );
                let inner = &value[6..value.len() - 1];
                let parts: Vec<&str> = inner.split_whitespace().collect();
                assert_eq!(parts.len(), 3, "Invalid OKLCH parts for {}: {}", key, value);
                for part in &parts {
                    assert!(
                        part.parse::<f32>().is_ok(),
                        "Invalid number in OKLCH for {}: {}",
                        key,
                        part
                    );
                }
            }
        }
    }

    #[test]
    fn test_color_constants_case_sensitivity() {
        let constants = color_constants();
        // Keys should be lowercase
        for key in constants.keys() {
            assert_eq!(key, &key.to_lowercase(), "Key should be lowercase: {}", key);
        }
    }

    #[test]
    fn test_color_constants_specific_values() {
        let constants = color_constants();
        // Test some known values
        assert_eq!(
            constants.get("red-500").unwrap(),
            "oklch(0.637 0.237 25.331)"
        );
        assert_eq!(
            constants.get("blue-500").unwrap(),
            "oklch(0.623 0.214 259.815)"
        );
        assert_eq!(
            constants.get("green-500").unwrap(),
            "oklch(0.723 0.219 149.579)"
        );
        assert_eq!(constants.get("black").unwrap(), "#000000");
        assert_eq!(constants.get("white").unwrap(), "#ffffff");
    }

    #[test]
    fn test_color_constants_all_families_present() {
        let constants = color_constants();
        let families = [
            "red", "orange", "amber", "yellow", "lime", "green", "emerald", "teal", "cyan", "sky",
            "blue", "indigo", "violet", "purple", "fuchsia", "pink", "rose", "slate", "gray",
            "zinc", "neutral", "stone",
        ];
        for family in &families {
            let key = format!("{}-500", family);
            assert!(constants.contains_key(&key), "Missing family: {}", family);
        }
    }
}
