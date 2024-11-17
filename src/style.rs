use stylers::style;

pub fn 樣式() -> &'static str {
    let styler_class = style! {
        :deep(kbd) {
            font-family: Fantasque Sans Mono, Inter, monospace;
        }
        :deep(.text-box) {
            display: flex;
            align-items: center;
            justify-content: center;
            margin: 24px;
            min-height: 120px;
        }
        :deep(.caption) {
            //border: 1px dotted;
            //border-radius: 32px;
            color: gray;
            font-family: LXGW WenKai, Inter, sans-serif;
            font-size: 3.5rem;
            padding: 16px 32px;
            text-align: center;
            width: fit-content;
        }
        :deep(.caption .accepted) {
            color: teal;
        }
        :deep(.caption .highlight) {
            color: purple;
        }
        :deep(.echo-bar) {
            display: flex;
            flex-direction: row;
            justify-content: center;
            gap: 16px;
            margin: 24px 0;
            height: 80px;
        }
        :deep(.input-code) {
            display: flex;
            justify-content: space-evenly;
            align-items: center;
            gap: 24px;
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
        :deep(.input-code.success) {
            background-color: color-mix(in srgb, teal 15%, transparent);
            border-color: teal;
            color: teal;
        }
        :deep(.raw-input) {
            flex: 1;
            text-align: right;
        }
        :deep(.translated-input, .lookup-code, .excercises) {
            flex: 1;
            font-family: LXGW WenKai, Inter, sans-serif;
            font-size: 2rem;
        }
        :deep(.translated-input) {
            overflow: visible;
            overflow-wrap: break-word;
            white-space: nowrap;
        }
        :deep(.lookup-code, .excercises) {
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
        :deep(.function.key, .function.key.pressed) {
            color: white;
            background-color: black;
            opacity: 15%;
        }
        :deep(.function.key.keydown) {
            color: white;
            background-color: black;
            opacity: 33%;
        }
        :deep(.space.key) {
            border-radius: 10% / 30%;
            height: 80px;
            width: 384px;
        }
         :deep(.key.hint) {
            color: green;
            background-color: color-mix(in srgb, green 10%, transparent);
        }
        :deep(.key.pressed) {
            color: purple;
            background-color: color-mix(in srgb, purple 10%, transparent);
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
        :deep(.key.hidden) {
            visibility: hidden;
        }
        :deep(.key .label) {
            font-size: 2.5rem;
            margin: auto;
            vertical-align: middle;
        }
        :deep(.function.key .label) {
            font-size: 1.2rem;
        }
    };
    styler_class
}
