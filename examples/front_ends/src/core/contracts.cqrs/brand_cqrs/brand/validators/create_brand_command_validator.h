// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/create_brand_dto.h"

#include "repository/interface_brand_repository.h"

#include "result.h"

using namespace;

using namespace FrontEnds::Contracts::Repository;

using namespace FrontEnds::Contracts::DTO::Brand;

namespace FrontEnds::Contracts::CQRS::Brand::Validators
{
class CreateBrandCommandValidator
{
public:
    CreateBrandCommandValidator(InterfaceBrandRepository *brandRepository)
        : m_brandRepository(brandRepository)
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
} // namespace FrontEnds::Contracts::CQRS::Brand::Validators