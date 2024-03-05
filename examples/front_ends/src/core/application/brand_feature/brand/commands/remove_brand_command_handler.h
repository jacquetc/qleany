// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "brand/commands/remove_brand_command.h"
#include "front_ends_example_application_brand_export.h"

#include "repository/interface_brand_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Brand;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Brand::Commands;

namespace FrontEnds::Application::Features::Brand::Commands
{
class FRONT_ENDS_EXAMPLE_APPLICATION_BRAND_EXPORT RemoveBrandCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemoveBrandCommandHandler(InterfaceBrandRepository *repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemoveBrandCommand &request);
    Result<int> restore();

  signals:
    // repositories handle remove signals
    // void brandRemoved(int id);

  private:
    InterfaceBrandRepository *m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemoveBrandCommand &request);
    Result<int> restoreImpl();
    FrontEnds::Entities::Brand m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Brand::Commands