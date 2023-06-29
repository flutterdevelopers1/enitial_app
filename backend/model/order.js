// need to update the order schemea for more updated oredered status.

// need to be running it on a thread.

const mongoose = require("mongoose");
const { productSchema } = require("./product");


// TODO: update the order schema
const orderSchema = mongoose.Schema({
  products: [
    {
      product: productSchema,
      quantity: {
        type: Number,
        required: true,
      },
    },
  ],
  totalPrice: {
    type: Number,
    required: true,
  },
  address: {
    type: String,
    required: true,
  },
  userId: {
    required: true,
    type: String,
  },
  orderedAt: {
    type: Number,
    required: true,
  },
  status: {
    type: Number,
    default: 0,
  },
//   numberOfOrder: {
//     type: Number,
//     default: 1
//   }
});

const Order = mongoose.model("Order", orderSchema);
module.exports = Order;