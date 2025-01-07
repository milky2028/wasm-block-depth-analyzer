import { TextLineStream } from "@std/streams/text-line-stream";

const file_path = Deno.args.at(0);
if (!file_path) {
  throw new Error("No path argument specified.");
}

type FunctionData = {
  name: string;
  start: number;
  end: number;
  length: number;
  try_statements: number;
  max_depth: number;
};

let current_line = 1;
let name = "";
let last_function_start: number | undefined = undefined;
let try_statements = 0;
let max_depth = 0;

const functions: FunctionData[] = [];
const file = await Deno.open(file_path);

await file.readable
  .pipeThrough(new TextDecoderStream())
  .pipeThrough(new TextLineStream()) // Each line in the file is piped to one item in the stream
  .pipeTo(
    new WritableStream({
      write(line) {
        // Every function is defined at the top level of the module, there are no nested functions.
        // Example function definition: (func $SetModelInfo_cr_host&__cr_negative&__CameraModelID_ (type 2) (param i32 i32) ...)
        const this_line_starts_function = line.includes("(func ");
        if (this_line_starts_function && last_function_start === undefined) {
          last_function_start = current_line;
        } else if (last_function_start !== undefined) {
          if (this_line_starts_function) {
            const end = current_line - 1;
            const length = end - last_function_start;
            functions.push({
              name,
              start: last_function_start,
              end: current_line - 1,
              length,
              try_statements,
              max_depth,
            });

            // function names start with (func $my_function_name and are followed by a space.
            name = line.match(/\(func\s(\$\S+)\s/)?.[1] ?? "";
            last_function_start = current_line;
            max_depth = 0;
            try_statements = 0;
          }

          // try blocks are followed by a block depth label.
          // Example try block start: try  ;; label = @2
          const line_has_try_block = line.includes("try  ;;");
          if (line_has_try_block) {
            // try blocks are labeled with their block depth
            const block_depth = +(line.match(/label = @(\d+)/)?.[1] ?? 0);
            max_depth = Math.max(block_depth, max_depth);
            try_statements++;
          }
        }

        current_line++;
      },
      close() {
        // we're missing the last function
      },
    })
  );

console.log(
  JSON.stringify(
    functions.sort((a, b) => b.try_statements - a.try_statements),
    null,
    2
  )
);
