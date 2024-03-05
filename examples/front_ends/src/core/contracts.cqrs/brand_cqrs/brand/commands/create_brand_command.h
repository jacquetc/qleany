// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/create_brand_dto.h"

namespace FrontEnds::Contracts::CQRS::Brand::Commands
{
class CreateBrandCommand
{
  public:
    CreateBrandCommand()
    {
    }

    FrontEnds::Contracts::DTO::Brand::CreateBrandDTO req;
};
} // namespace FrontEnds::Contracts::CQRS::Brand::Commands