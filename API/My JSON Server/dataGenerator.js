
// database-faker.js
let faker = require('faker');
let generateData = () => {
    let data = []
    for (let id = 0; id < 100; id++) {
        data.push({
            "id": id,
            "address": faker.address.streetAddress(),
            "country": faker.address.country(),
            "first_name": faker.name.firstName(),
            "last_name": faker.name.lastName(),
            "phone": faker.phone.phoneNumber(),
            "word": faker.lorem.word(),
        });
    }
    return { "data": data }
}
module.exports = generateData
