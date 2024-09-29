// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "brand/commands/create_brand_command.h"
#include "front_ends_example_application_brand_export.h"
#include "repository/interface_brand_repository.h"
#include "result.h"
#include <QPromise>

using namespace FrontEnds;
using namespace FrontEnds::Entities;
using namespace FrontEnds::Contracts::DTO::Brand;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Brand::Commands;

namespace FrontEnds::Application::Features::Brand::Commands
{
class FRONT_ENDS_EXAMPLE_APPLICATION_BRAND_EXPORT CreateBrandCommandHandler : public QObject
{
    Q_OBJECT
public:
    CreateBrandCommandHandler(InterfaceBrandRepository *repository);

    Result<BrandDTO> handle(QPromise<Result<void>> &progressPromise, const CreateBrandCommand &request);
    Result<BrandDTO> restore();

Q_SIGNALS:
    void brandCreated(FrontEnds::Contracts::DTO::Brand::BrandDTO brandDto);
    void brandRemoved(int id);

    void relationWithOwnerInserted(int id, int ownerId, int position);
    void relationWithOwnerRemoved(int id, int ownerId);

private:
    InterfaceBrandRepository *m_repository;
    Result<BrandDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateBrandCommand &request);
    Result<BrandDTO> restoreImpl();
    Result<FrontEnds::Entities::Brand> m_newEntity;

    int m_ownerId = -1;
    int m_position = -1;

    FrontEnds::Entities::Brand m_oldOwnerBrand;
    FrontEnds::Entities::Brand m_ownerBrandNewState;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace FrontEnds::Application::Features::Brand::Commands