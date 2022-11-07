const Surreal = require('surrealdb.js').default;
console.log( Surreal );

// const db = new Surreal('http://localhost:3000/rpc');
const db = new Surreal('http://127.0.0.1:8000/rpc');

async function main() {

	try {

        console.log( db );

		let jwt = await db.signup( {
			NS: 'test',
			DB: 'test',
			SC: 'allusers',
			email: 'joey',
			pass: 'poop'
		} );

		console.log( jwt );
		return;

		// Signin as a namespace, database, or root user
		await db.signin({
			user: 'root',
			pass: 'root',
		});

		// Select a specific namespace / database
		await db.use('test', 'test');

		// // Create a new person with a random id
		let created = await db.create("person", {
			title: 'Founder & CEO',
			name: {
				first: 'Tobie',
				last: 'Morgan Hitchcock',
			},
			marketing: true,
			identifier: Math.random().toString(36).substr(2, 10),
		});

		// // Update a person record with a specific id
		// let updated = await db.change("person:jaime", {
		// 	marketing: true,
		// });

		// // Select all people records
		// let people = await db.select("person");

		// // Perform a custom advanced query
		// let groups = await db.query('SELECT marketing, count() FROM type::table($tb) GROUP BY marketing', {
		// 	tb: 'person',
		// });

		// console.log( updated, people, groups, created)


	} catch (e) {

		console.error('ERROR', e);

	}

}

main();