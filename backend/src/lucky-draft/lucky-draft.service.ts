import { Injectable } from '@nestjs/common';
import { Cron, CronExpression } from '@nestjs/schedule';

@Injectable()
export class LuckyDraftService {
  @Cron(CronExpression.EVERY_SECOND)
  handleDraft() {
    const luckyNumbers = this.generateLotteryNumbers();
    // @todo:
    // - Look up in the database for tickets that have the status set to "Created"
    // - Verify the numbers and check their winning amount.
    // - Interact with the luckySOL contract to airdrop the rewards.
  }

  private generateRandomNumber(x = 0, y = 49) {
    if (x > y) {
      throw new Error(
        'The first parameter (x) must be less than or equal to the second parameter (y).',
      );
    }
    return Math.floor(Math.random() * (y - x + 1)) + x;
  }

  private generateLotteryNumbers() {
    const x = 0;
    const y = 49;
    const luckyNumbers = new Set<number>();
    while (luckyNumbers.size !== 7) {
      luckyNumbers.add(this.generateRandomNumber(x, y));
    }
    return Array.from(luckyNumbers);
  }
}
