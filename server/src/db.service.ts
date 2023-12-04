import { Injectable } from '@nestjs/common';
import { PrismaService } from './prisma.service';
import { Light, Prisma, Animation } from '@prisma/client';

@Injectable()
export class DBService {
  constructor(private prisma: PrismaService) {}

  async getAllCords(): Promise<Light[]> {
    return await this.prisma.light.findMany();
  }

  async updateCords(
    where: Prisma.LightWhereUniqueInput,
    data: Prisma.LightUpdateInput,
  ) {
    return this.prisma.light.update({
      data,
      where,
    });
  }

  async getAllAnims() {
    return await this.prisma.animation.findMany();
  }
}
