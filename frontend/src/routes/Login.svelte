<script lang="ts">
    // const Surreal = require('surrealdb.js').default;
    import Surreal from 'surrealdb.js';
    const db = new Surreal('http://127.0.0.1:3000/rpc');

    console.log( Surreal );

    let username = "Joey";
    let password = "";
    let response = "";

    // async function logIn()
    // {
    //     let query = `{
    //         userValidate(
    //             user: "${username}", 
    //             password: "${password}"
    //         )
    //     }`;

    //     // return await fetch( 'https://dev.meadowserver.com/graphql', {
    //     //     method: 'POST',
    //     //     headers: { 'Content-Type': 'application/json' },
    //     //     credentials: 'include',
    //     //     body: JSON.stringify( { query } ),
    //     // } ).then( resp => {
    //     //     return resp.json()
    //     // });
    // }

    function handleClick() {
        db.signin( {
            NS: 'parrhesia',
            DB: 'parrhesia',
            SC: 'allusers',
            user: 'joey123',
            pass: 'poop',
        }).then( value => {
            console.log( value );
        })
    }

    function signUp() {
        db.signup( {
			NS: 'parrhesia',
			DB: 'parrhesia',
			SC: 'allusers',
			user: 'joey123',
			pass: 'poop'
		} ).then( value => {
            console.log( value );
        })
    }
    
    async function addThing() {
        // Select a specific namespace / database
        await db.select("user");
        
        // await db.use( 'parrhesia', 'parrhesia' );
        
        // await db.create( "person", {
        //     name: "Joey",
        //     age: 24
        // });
        
        // await db.select( "person" );
		// await db.use('test', 'test');

		// Create a new person with a random id
		// let created = await db.create("person", {
		// 	title: 'Founder & CEO',
		// 	name: {
		// 		first: 'Tobie',
		// 		last: 'Morgan Hitchcock',
		// 	},
		// 	marketing: true,
		// 	identifier: Math.random().toString(36).substr(2, 10),
		// });
    }
    
    function handleThing() {
        addThing().then( resp => {
            console.log( resp );
        })
    }

    function log() {
        console.log( "Username", username );
        console.log( "Password", password );
    }
</script>

<template>
    <div style="display:flex; flex-direction: column; align-items:center; justify-content:center; width: 100%; height: 100%">
        <div style="display:flex; flex-direction: column; gap: 10px;">
            <p>{response}</p>
            <input bind:value={ username } on:change={ log } type="text" />
            <br>
            <input bind:value={ password } type="password" />
            <br>
            <button on:click={ handleClick }>Log in</button>
            <button on:click={ signUp }>Sign up</button>
            <button on:click={ handleThing }>Hi there</button>
        </div>
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