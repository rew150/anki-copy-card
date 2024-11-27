<script lang="ts">
  import { fetch } from "@tauri-apps/plugin-http";
  import sanitizeHtml from 'sanitize-html';
  interface Field {
    value: string
    order: number
  }
  type GuiCurrentCard = null | {
    result: {
      fields: {
        Kanji: Field,
        Kana: Field,
        SentenceFront: Field,
        SentenceBack: Field,
        Picture: Field,
        KankenAudio: Field,
        KankenLevel: Field,
        Meaning: Field,
        Diagram: Field,
      },
      deckName: string,
    },
    error: any
  }

  let msg = $state("");
  let kanji = $state("");
  let kana = $state("");
  let front = $state("");
  let back = $state("");
  const deckname = "漢字 Writing"

  function onKanjiKana(e: Event) {
    front = `${kanji}[${kana}]`
  }

  async function fire(_event: Event) {
    _event.preventDefault()
    msg = "";
    const response = await fetch('http://localhost:8765', {
      method: 'POST',
      body: JSON.stringify({
        action: "guiCurrentCard",
        version: 6,
      })
    });
    console.log(response.status)
    const cardData: GuiCurrentCard = await response.json();
    if (!cardData) {
      msg = "card data missing"
      return
    }
    if (cardData.error) {
      msg = `card data error ${""+cardData.error}`
      return
    }
    const fields = cardData.result.fields;
    const sentence = sanitizeHtml(fields.SentenceBack.value, {allowedTags: []});
    const res2 = await fetch("http://localhost:8765", {
      method: "POST",
      body: JSON.stringify({
        action: "guiAddCards",
        version: 6,
        params: {
          note: {
            deckName: "Immersion",
            modelName: "Immersion",
            fields: {
              Front: front || `${fields.Kanji.value}[${fields.Kana.value}]`,
              Back: back || fields.Meaning.value,
              "Back Paragraph": `${sentence}<br />${fields.Picture.value}`,
              AudioGuide: kanji || fields.Kanji.value,
              Audio: fields.KankenAudio.value,
            },
            tags: [
              "Immersion",
              "from::KanKenDeck"
            ],
          }
        }
      })
    })
    kanji = ""
    kana = ""
    front = ""
    back = ""
  }

  async function handleBack(_event: Event) {
    _event.preventDefault()
    const response = await fetch('http://localhost:8765', {
      method: 'POST',
      body: JSON.stringify({
        action: "guiDeckReview",
        version: 6,
        params: {
          name: deckname 
        }
      })
    });
  }

</script>

<div>Custom Kanji: <br /><input bind:value={kanji} onchange={onKanjiKana} /></div>
<div>Custom Kana: <br /><input bind:value={kana} onchange={onKanjiKana} /></div>
<div>Front: <br /><input bind:value={front} /></div>
<div>Back: <br /><input bind:value={back} /></div>

<button onclick={fire} >Fire</button>

<div>Return to deck "{deckname}": <button onclick={handleBack}>return</button></div>

{msg}
