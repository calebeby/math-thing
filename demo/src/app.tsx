import { useComputed, useSignal } from "@preact/signals";
import katex from "katex";
import "katex/contrib/copy-tex";
import "katex/dist/katex.min.css";
import "./app.css";

export function App() {
  const mathRaw = useSignal(
    // String.raw`\cos(\theta)=\frac{e^{i\theta}+e^{-i\theta}}{2}`
    // String.raw`-\frac{\hbar^2}{2m}\frac{\partial^2\psi}{\partial x^2} = i\hbar\frac{\partial \psi}{\partial t}`
    String.raw`-\frac{\hbar^2}{2m}\htmlClass{hl}{\frac{\partial^2\psi}{\partial x^2}} = \htmlClass{hl}{i\hbar}\frac{\partial \psi}{\htmlClass{hl}{\partial t}}`
  );

  const mathOutput = useComputed(() => {
    try {
      const result = katex.renderToString(mathRaw.value, {
        throwOnError: true,
        displayMode: true,
        trust: true,
      });
      return { success: true, data: result } as const;
    } catch (error) {
      return {
        success: false,
        data: error as import("katex").ParseError,
      } as const;
    }
  });

  return (
    <div class="app">
      <input
        class="math-input"
        value={mathRaw}
        onInput={(el) => {
          mathRaw.value = el.currentTarget.value;
        }}
      />
      <div class="math-output">
        {mathOutput.value.success ? (
          <div dangerouslySetInnerHTML={{ __html: mathOutput.value.data }} />
        ) : (
          <>
            <div>{mathOutput.value.data.message}</div>
            <pre>
              <code>{`${mathRaw}
${" ".repeat(mathOutput.value.data.position)}^`}</code>
            </pre>
          </>
        )}
      </div>
    </div>
  );
}
