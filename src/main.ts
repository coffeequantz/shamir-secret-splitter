import './styles.css'
import { invoke } from "@tauri-apps/api/core";



document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <h1>Shamir Secret Splitter</h1>

  <section>
    <textarea id="secret" rows="6" placeholder="Enter long secret"></textarea>
    <input id="shares" type="number" value="5" placeholder="Shares" />
    <input id="threshold" type="number" value="3" placeholder="Threshold" />
    <button id="splitBtn">Split</button>
    <pre id="output"></pre>
  </section>

  <section>
    <h2>Recombine</h2>
    <textarea id="sharelist" rows="6" placeholder="Paste shares (one per line)"></textarea>
    <button id="combineBtn">Combine</button>
    <pre id="combined"></pre>
  </section>
`


document.getElementById("splitBtn")!.addEventListener("click", async () => {
  const secret = (document.getElementById("secret") as HTMLTextAreaElement).value
  const shares = parseInt((document.getElementById("shares") as HTMLInputElement).value)
  const threshold = parseInt((document.getElementById("threshold") as HTMLInputElement).value)

  try {
    const result: string[] = await invoke("split", { secret, shares, threshold })
    document.getElementById("output")!.innerText = JSON.stringify(result, null, 2)
  } catch (err) {
    document.getElementById("output")!.innerText = "❌ " + err
  }
})

document.getElementById("combineBtn")!.addEventListener("click", async () => {
  const lines = (document.getElementById("sharelist") as HTMLTextAreaElement).value
    .split("\n").map(l => l.trim()).filter(Boolean)

  try {
    const result: string = await invoke("combine", { shares: lines })
    document.getElementById("combined")!.innerText = result
  } catch (err) {
    document.getElementById("combined")!.innerText = "❌ " + err
  }
})
