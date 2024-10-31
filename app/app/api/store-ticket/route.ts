import {NextResponse} from 'next/server';
import * as crypto from 'node:crypto';
import {MerkleTree} from 'merkletreejs';

function sha256(data:crypto.BinaryLike) {
    return crypto.createHash("sha256").update(data);
}

export const config = {
  api: {
    bodyParser: true
  }
};

export default function POST() {
  const leafes = ["a", "b", "c"];
  const tree = new MerkleTree(leafes, sha256);
  tree.getMultiProof();
  NextResponse.json({message: "Hello"})
}