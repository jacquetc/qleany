// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "brand/update_brand_dto.h"


#include "repository/interface_brand_repository.h"

#include "qleany/common/result.h"

using namespace Qleany;

using namespace Simple::Contracts::Repository;

using namespace Simple::Contracts::DTO::Brand;

namespace Simple::Contracts::CQRS::Brand::Validators
{
class UpdateBrandCommandValidator
{
  public:
    UpdateBrandCommandValidator(InterfaceBrandRepository *brandRepository)
        :  m_brandRepository(brandRepository)
    {
    }

    Result<void> validate(const UpdateBrandDTO &dto) const

    {




        Result<bool> existsResult = m_brandRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }




        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceBrandRepository *m_brandRepository;

};
} // namespace Simple::Contracts::CQRS::Brand::Validators