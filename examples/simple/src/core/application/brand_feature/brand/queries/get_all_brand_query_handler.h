// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_brand_export.h"
#include "brand/brand_dto.h"

#include "repository/interface_brand_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::Repository;

namespace Simple::Application::Features::Brand::Queries
{
class SIMPLEEXAMPLE_APPLICATION_BRAND_EXPORT GetAllBrandQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllBrandQueryHandler(InterfaceBrandRepository *repository);
    Result<QList<BrandDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    InterfaceBrandRepository *m_repository;
    Result<QList<BrandDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Brand::Queries