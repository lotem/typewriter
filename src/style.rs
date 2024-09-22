use stylers::style;

pub fn 樣式() -> &'static str {
    let styler_class = style! {
        :deep(kbd) {
            font-family: Fantasque Sans Mono, Inter, monospace;
        }
        :deep(.input-code) {
            display: flex;
            justify-content: space-evenly;
            align-items: center;
            gap: 24px;
            margin: 24px auto 16px auto;
            width: 384px;
            height: 80px;
            border: 2px solid green;
            border-radius: 24px;
        }
        :deep(.input-code .raw-input) {
            flex: 1;
            color: green;
            font-size: 2.5rem;
            text-align: right;
        }
        :deep(.input-code .translated-input) {
            flex: 1;
            color: green;
            font-family: LXGW WenKai, Inter, sans-serif;
            font-size: 2rem;
        }
        :deep(.board) {
            display: flex;
            flex-direction: column;
            justify-content: center;
            gap: 16px;
        }
        :deep(.row) {
            display: flex;
            flex-direction: row;
            justify-content: center;
            gap: 16px;
        }
        :deep(.key) {
            border: 3px solid;
            border-radius: 40% 40% 20% 20% / 50% 50% 30% 30%;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 80px;
            width: 80px;
        }
        :deep(.key.space) {
            border-radius: 10% / 30%;
            height: 80px;
            width: 384px;
        }
        :deep(.key.pressed) {
            color: purple;
            background-color: color-mix(in srgb, purple 5%, transparent);
        }
        :deep(.key.keydown) {
            color: purple;
            background-color: color-mix(in srgb, purple 25%, transparent);
        }
        :deep(.key.fallback, .key.empty) {
            opacity: 33%;
        }
        :deep(.key .label) {
            font-size: 2.5rem;
            margin: auto;
            vertical-align: middle;
        }
    };
    styler_class
}
