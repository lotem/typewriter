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
            border: 2px solid;
            border-radius: 24px;
            font-size: 2.5rem;
        }
        :deep(.input-code.freeplay) {
            border-color: purple;
            color: purple;
        }
        :deep(.input-code.target) {
            border-color: green;
            color: green;
        }
        :deep(.raw-input) {
            flex: 1;
            text-align: right;
        }
        :deep(.translated-input, .lookup-code) {
            flex: 1;
            font-family: LXGW WenKai, Inter, sans-serif;
            font-size: 2rem;
        }
        :deep(.lookup-code) {
            border: 1px dotted green;
            border-radius: 10px;
            margin: auto 18px;
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
         :deep(.key.hint) {
            color: green;
            background-color: color-mix(in srgb, green 5%, transparent);
        }
        :deep(.key.pressed) {
            color: purple;
            background-color: color-mix(in srgb, purple 5%, transparent);
        }
         :deep(.key.hint.pressed) {
            color: teal;
            background-color: color-mix(in srgb, teal 25%, transparent);
        }
        :deep(.key.keydown, .key.hint.keydown) {
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
