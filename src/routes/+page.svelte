<script lang="ts">
  import sanitizeHtml from 'sanitize-html';
  import { invoke } from '@tauri-apps/api/core';
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

  async function anki(action: string, params: any = undefined): Promise<any> {
    let res_json: any = await invoke("anki_request", {action, ...(params === undefined ? {} : {params})})
    return res_json
  }

  async function fire(_event: Event) {
    _event.preventDefault()
    msg = "";
    back = back.replaceAll("\n", "<br />");
    let cardData: GuiCurrentCard;
    try {
      cardData = await anki("guiCurrentCard");
    } catch (error) {
      msg = ""+ error;
      return
    }
    if (!cardData) {
      return
    }

    console.log(cardData)
    const fields = cardData.result.fields;
    const sentence = sanitizeHtml(fields.SentenceBack.value, {allowedTags: []});
    try {
      await anki("guiAddCards", {
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
      })
    } catch (error) {
      msg = ""+error
      return
    }
    kanji = ""
    kana = ""
    front = ""
    back = ""
  }

  async function handleBack(_event: Event) {
    _event.preventDefault()
    await anki('guiDeckReview', {name: deckname})
  }

</script>

<div>Custom Kanji: <br /><input bind:value={kanji} onchange={onKanjiKana} /></div>
<div>Custom Kana: <br /><input bind:value={kana} onchange={onKanjiKana} /></div>
<div>Front: <br /><input bind:value={front} /></div>
<div>Back: <br /><textarea bind:value={back}></textarea></div>

<button onclick={fire} >Fire</button>

<div>Return to deck "{deckname}": <button onclick={handleBack}>return</button></div>

{msg}
