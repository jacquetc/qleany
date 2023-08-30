// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_brand_export.h"
#include "brand/brand_dto.h"
#include "brand/commands/create_brand_command.h"
#include "qleany/common/result.h"
#include "repository/interface_brand_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Domain;
using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Brand::Commands;

namespace Simple::Application::Features::Brand::Commands
{
class SIMPLEEXAMPLE_APPLICATION_BRAND_EXPORT CreateBrandCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateBrandCommandHandler(InterfaceBrandRepository *repository);

    Result<BrandDTO> handle(QPromise<Result<void>> &progressPromise, const CreateBrandCommand &request);
    Result<BrandDTO> restore();

  signals:
    void brandCreated(Simple::Contracts::DTO::Brand::BrandDTO brandDto);
    void brandRemoved(int id);

  private:
    InterfaceBrandRepository *m_repository;
    Result<BrandDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateBrandCommand &request);
    Result<BrandDTO> restoreImpl();
    Result<Simple::Domain::Brand> m_newEntity;

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Brand::Commands