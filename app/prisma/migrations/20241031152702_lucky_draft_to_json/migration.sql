/*
  Warnings:

  - You are about to alter the column `lucky_draft` on the `Ticket` table. The data in that column could be lost. The data in that column will be cast from `VarChar(191)` to `Json`.

*/
-- AlterTable
ALTER TABLE `Ticket` MODIFY `lucky_draft` JSON NOT NULL;
