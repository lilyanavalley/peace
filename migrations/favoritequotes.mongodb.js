/* global use, db */
// MongoDB Playground
// Use Ctrl+Space inside a snippet or a string literal to trigger completions.

const database = 'profile';
const collection = 'quotes';

// Create a new database.
use(database);

// Create a new collection.
db.createCollection(collection);

db.collection.insertMany([
  {
    quotation:    "What the sigma?",
    citation: "Girlfriend, [Brainrot Girlfriend](https://mangadex.org/title/d0ddd740-4b91-4c66-bfd0-b36a77f8e730/brainrot-girlfriend)"
  },
  {
    quotation:
`**Girlfriend**: *"Ligma, Ligma. Sugma, Sugma. Sawcon, Sawcon. Dragon, Dragon. Aloaf, Aloaf."*
**Boyfriend**: *"Ha~ nice try but- wait, what comes after 'a loaf'?"
"A loaf u~"
"Damn it, she got me this time."*`,
    citation: "Boyfriend and Girlfriend, [Brainrot Girlfriend](https://mangadex.org/title/d0ddd740-4b91-4c66-bfd0-b36a77f8e730/brainrot-girlfriend)"
  }
]);

// The prototype form to create a collection:
/* db.createCollection( <name>,
  {
    capped: <boolean>,
    autoIndexId: <boolean>,
    size: <number>,
    max: <number>,
    storageEngine: <document>,
    validator: <document>,
    validationLevel: <string>,
    validationAction: <string>,
    indexOptionDefaults: <document>,
    viewOn: <string>,
    pipeline: <pipeline>,
    collation: <document>,
    writeConcern: <document>,
    timeseries: { // Added in MongoDB 5.0
      timeField: <string>, // required for time series collections
      metaField: <string>,
      granularity: <string>,
      bucketMaxSpanSeconds: <number>, // Added in MongoDB 6.3
      bucketRoundingSeconds: <number>, // Added in MongoDB 6.3
    },
    expireAfterSeconds: <number>,
    clusteredIndex: <document>, // Added in MongoDB 5.3
  }
)*/

// More information on the `createCollection` command can be found at:
// https://www.mongodb.com/docs/manual/reference/method/db.createCollection/
