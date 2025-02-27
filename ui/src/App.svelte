<script lang="ts">
  import { onMount } from 'svelte';

  interface Field {
    name: string;
    required: boolean;
    value: { Info?: string } | string;
  }

  let fields: Field[] = [];

  async function fetchForm() {
    const response = await fetch('http://localhost:3000/authorize', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      credentials: 'include', // Include credentials in the request
      body: JSON.stringify({ fields: [] })
    });

    const result = await response.json();
    fields = result.fields;
  }


  async function handleSubmit(event: Event) {
    event.preventDefault();

    const formData = new FormData(event.target as HTMLFormElement);
    const data = { fields: [] as { name: string; required: boolean; value: {String: string} }[] };

    formData.forEach((value, key) => {
      data.fields.push({
        name: key,
        required: true,
        value: {
          String: value as string,
        }
      });
    });


    const response = await fetch('http://localhost:3000/authorize', { // Modification ici
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      credentials: 'include', // Include credentials in the request
      body: JSON.stringify(data)
    });

    const result = await response.json();
    fields = result.fields;
  }

  onMount(fetchForm);
</script>

<main>
  <form on:submit={handleSubmit}>
    {#each fields as field}
      <div>
        <label for={field.name}>{field.name}</label>
        {#if typeof field.value === 'object' && field.value.Info}
          <span>{field.value.Info}</span>
        {:else}
          <input type="text" id={field.name} name={field.name} required={field.required} />
        {/if}
      </div>
    {/each}
    <button type="submit">Submit</button>
  </form>
</main>

<style>
  main {
    padding: 1em;
    max-width: 600px;
    margin: 0 auto;
  }
  div {
    margin-bottom: 1em;
  }
  label {
    display: block;
    margin-bottom: 0.5em;
  }
  input {
    width: 100%;
    padding: 0.5em;
    box-sizing: border-box;
  }
  button {
    padding: 0.5em 1em;
  }
</style>