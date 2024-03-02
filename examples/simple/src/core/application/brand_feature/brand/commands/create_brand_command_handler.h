// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "brand/commands/create_brand_command.h"
#include "repository/interface_brand_repository.h"
#include "simple_example_application_brand_export.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Simple::Entities;
using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Brand::Commands;

namespace Simple::Application::Features::Brand::Commands
{
class SIMPLE_EXAMPLE_APPLICATION_BRAND_EXPORT CreateBrandCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateBrandCommandHandler(InterfaceBrandRepository *repository);

    Result<BrandDTO> handle(QPromise<Result<void>> &progressPromise, const CreateBrandCommand &request);
    Result<BrandDTO> restore();

  signals:
    void brandCreated(Simple::Contracts::DTO::Brand::BrandDTO brandDto);
    void brandRemoved(int id);

    void relationWithOwnerInserted(int id, int ownerId, int position);
    void relationWithOwnerRemoved(int id, int ownerId);

  private:
    InterfaceBrandRepository *m_repository;
    Result<BrandDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateBrandCommand &request);
    Result<BrandDTO> restoreImpl();
    Result<Simple::Entities::Brand> m_newEntity;

    int m_ownerId = -1;
    int m_position = -1;

    Simple::Entities::Brand m_oldOwnerBrand;
    Simple::Entities::Brand m_ownerBrandNewState;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace Simple::Application::Features::Brand::Commands