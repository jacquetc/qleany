// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "brand/commands/update_brand_command.h"
#include "simple_example_application_brand_export.h"

#include "repository/interface_brand_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Brand::Commands;

namespace Simple::Application::Features::Brand::Commands
{
class SIMPLE_EXAMPLE_APPLICATION_BRAND_EXPORT UpdateBrandCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateBrandCommandHandler(InterfaceBrandRepository *repository);
    Result<BrandDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateBrandCommand &request);
    Result<BrandDTO> restore();

  Q_SIGNALS:
    void brandUpdated(Simple::Contracts::DTO::Brand::BrandDTO brandDto);
    void brandDetailsUpdated(int id);

  private:
    InterfaceBrandRepository *m_repository;
    Result<BrandDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateBrandCommand &request);
    Result<BrandDTO> restoreImpl();
    Result<BrandDTO> saveOldState();
    Result<BrandDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Brand::Commands