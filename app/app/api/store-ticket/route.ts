import {NextRequest, NextResponse} from 'next/server';
import * as crypto from 'node:crypto';
import {PrismaClient} from '@prisma/client';
import {Buffer} from 'node:buffer';

function sha256(data:Buffer) {
  // console.log(data)
    return crypto.createHash("sha256").update(Uint8ClampedArray.from(data));
}

function parseToBytes(msg: string) : Uint8ClampedArray {
  return Uint8ClampedArray.from(Buffer.from(msg))
}

const prisma = new PrismaClient()


export async function POST(request: NextRequest) {
  const req: {message: string} = await request.json();
  return NextResponse.json({message: "Work in progress .."})
}