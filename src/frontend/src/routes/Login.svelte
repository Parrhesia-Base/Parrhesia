<script lang="ts">
    let username = "Joey";
    let password = "";
    let response = "";
    // function increment() {
    //     value += 1;
    // }

    async function logIn()
    {
        let query = `{
            userValidate(
                user: "${username}", 
                password: "${password}"
            )
        }`;

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
        logIn().then( resp => {
            response = JSON.stringify( resp );
        })
    }

    function log() {
        console.log( "Username", username );
        console.log( "Password", password)
    }
</script>

<template>
    <div>
        <p>{response}</p>
        <input bind:value={ username } on:change={ log } type="text" />
        <br>
        <input bind:value={ password } type="password" />
        <br>
        <button on:click={ handleClick }>Log in</button>
    </div>
</template>

<style>
    /* button {
        padding: 12px, 12px, 12px, 12px;
        font-size: 200px;
        aspect-ratio: 1;
        height: 300px;
    } */
</style>