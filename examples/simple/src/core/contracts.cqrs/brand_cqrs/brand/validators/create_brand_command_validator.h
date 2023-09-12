// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "brand/create_brand_dto.h"


#include "repository/interface_brand_repository.h"

#include "qleany/common/result.h"

using namespace Qleany;

using namespace Simple::Contracts::Repository;

using namespace Simple::Contracts::DTO::Brand;

namespace Simple::Contracts::CQRS::Brand::Validators
{
class CreateBrandCommandValidator
{
  public:
    CreateBrandCommandValidator(InterfaceBrandRepository *brandRepository)
        :  m_brandRepository(brandRepository)
    {
    }

    Result<void> validate(const CreateBrandDTO &dto) const

    {




        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceBrandRepository *m_brandRepository;

};
} // namespace Simple::Contracts::CQRS::Brand::Validators