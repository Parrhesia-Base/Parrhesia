<script lang="ts">
    let value = 0;
    let response = "";

    function increment() {
        value += 1;
        handleClick( );
    }

    async function listUsers()
    {
        let query = `{ listUsers { id, username } }`;

        return await fetch( 'https://dev.meadowserver.com/graphql', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify( { query } ),
        } ).then( resp => {
            return resp.json()
        });
    }

    function handleClick() {
        listUsers().then( resp => {
            response = JSON.stringify( resp );
        });
    }
</script>

<template>
    <button on:click={ increment }>{ value }</button>
    <p>{response}</p>
</template>

<style>
    button {
        padding: 12px, 12px, 12px, 12px;
        font-size: 200px;
        aspect-ratio: 1;
        height: 300px;
    }
</style>